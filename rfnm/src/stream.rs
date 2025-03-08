use crate::RfnmApiError::BufferCountMismatch;
use crate::device::Device;
use crate::{RfnmApiError, check_code};
use rfnm_sys::{
    StreamWrapper,
    WrappedThrownError,
    device_set_stream_format,
    rfnm_channel,
    rfnm_stream_format,
    stream_create,
    stream_free,
    stream_read,
    stream_set_auto_dc_offset,
    stream_start,
    stream_stop,
};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::time::Duration;

use num_complex::Complex;

pub trait StreamDataFormat {
    fn api_format() -> rfnm_stream_format;
}

impl StreamDataFormat for Complex<i8> {
    fn api_format() -> rfnm_stream_format {
        rfnm_stream_format::STREAM_FORMAT_CS8
    }
}
impl StreamDataFormat for Complex<i16> {
    fn api_format() -> rfnm_stream_format {
        rfnm_stream_format::STREAM_FORMAT_CS16
    }
}
impl StreamDataFormat for Complex<f32> {
    fn api_format() -> rfnm_stream_format {
        rfnm_stream_format::STREAM_FORMAT_CF32
    }
}

/// A synchronized rx stream over one or more device channels
///
/// Takes ownership of the device
pub struct RxStream<T> {
    _p: PhantomData<T>,
    channel_count: usize,
    suggested_buffer_size: usize,
    device: Option<Device>,
    wrapper: *mut StreamWrapper,
}

pub struct StreamReadInfo {
    pub elements_read: usize,
    pub timestamp_ns: u64,
}

impl<T: StreamDataFormat> RxStream<T> {
    pub fn new(device: Device, channels: rfnm_channel) -> Result<Self, (RfnmApiError, Device)> {
        let channel_count = channels.0.count_ones() as usize;
        let mut thrown_err = WrappedThrownError::empty();
        let mut suggested_buffer_size = 0;
        if let Err(e) = check_code(unsafe {
            device_set_stream_format(
                device.wrapper(),
                T::api_format(),
                &mut suggested_buffer_size,
            )
        }) {
            return Err((e, device));
        }

        let wrapper = unsafe { stream_create(device.wrapper(), channels.0 as u8, &mut thrown_err) };
        if wrapper.is_null() {
            Err((thrown_err.into(), device))
        } else {
            Ok(Self {
                _p: PhantomData::default(),
                channel_count,
                suggested_buffer_size,
                device: Some(device),
                wrapper,
            })
        }
    }

    pub fn into_device(mut self) -> Device {
        // unwrap: move safe because are only ever crated by new, which always fills
        // avoid moving out of self here
        self.device.take().unwrap()
    }

    pub fn device(&self) -> &Device {
        // unwrap: safe because we only ever create with Some()
        self.device.as_ref().unwrap()
    }

    pub fn channel_count(&self) -> usize {
        self.channel_count
    }

    pub fn suggested_buffer_size(&self) -> usize {
        self.suggested_buffer_size
    }

    pub fn set_auto_dc_offset(&self, auto: bool, channel: rfnm_channel) {
        unsafe { stream_set_auto_dc_offset(self.wrapper, auto, channel.0 as u8) }
    }

    pub fn start(&self) -> Result<(), RfnmApiError> {
        check_code(unsafe { stream_start(self.wrapper) })
    }

    pub fn stop(&self) -> Result<(), RfnmApiError> {
        check_code(unsafe { stream_stop(self.wrapper) })
    }

    pub fn read<'a>(
        &self,
        dst: &[&'a mut [T]],
        timeout: Duration,
    ) -> Result<StreamReadInfo, RfnmApiError> {
        if dst.len() != self.channel_count {
            return Err(RfnmApiError::BufferCountMismatch(
                dst.len(),
                self.suggested_buffer_size,
            ));
        }
        if dst.len() > 1 {
            for buff in &dst[1..] {
                if buff.len() != dst[0].len() {
                    return Err(RfnmApiError::BufferSizeMismatch);
                }
            }
        }
        // does this really happen? I think the api fails us on no channels, or at least should?
        if dst.len() == 0 {
            return Err(BufferCountMismatch(0, 0));
        }

        let element_count = dst[0].len();
        let mut actually_written = 0;
        let mut timestamp = 0;
        let timeout_us = timeout.as_micros().min(u32::MAX as u128) as u32;
        // we need the raw places. We do not want to alloc in this path though
        // we *do* know that there cannot be more than 8 channels though, making this array more or less free.
        assert!(dst.len() <= 8);
        let mut raw_buffers = [std::ptr::null_mut(); 8];
        for i in 0..dst.len() {
            raw_buffers[i] = dst[i].as_ptr() as *mut c_void;
        }
        // perform the magic
        check_code(unsafe {
            stream_read(
                self.wrapper,
                raw_buffers.as_ptr(),
                element_count,
                &mut actually_written,
                &mut timestamp,
                timeout_us,
            )
        })?;
        Ok(StreamReadInfo {
            elements_read: actually_written,
            timestamp_ns: timestamp,
        })
    }
}

impl<T> Drop for RxStream<T> {
    fn drop(&mut self) {
        unsafe { stream_free(self.wrapper) };
    }
}
