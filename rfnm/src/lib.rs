pub mod channel_settings;
pub mod device;
pub mod hwinfo;
pub mod stream;

use crate::hwinfo::HwInfo;
use rfnm_sys::{WrappedThrownError, rfnm_api_failcode, rfnm_channel, rfnm_dev_hwinfo};
use std::ffi::CStr;
use std::mem::MaybeUninit;
use thiserror::Error;

/// Discover all connected rfnm devices.
pub fn discover_usb_boards() -> Vec<HwInfo> {
    let mut dst = Vec::new();
    unsafe {
        let board_count = rfnm_sys::find_usb_devices(std::ptr::null_mut(), 0);
        let mut dst_vec = vec![MaybeUninit::uninit(); board_count];
        let actual_count =
            rfnm_sys::find_usb_devices(dst_vec.as_mut_ptr() as *mut rfnm_dev_hwinfo, dst_vec.len());
        for i in 0..actual_count {
            let raw_hw_info: rfnm_dev_hwinfo = dst_vec[i].assume_init();
            dst.push(raw_hw_info.into())
        }
    }

    dst
}

#[derive(Debug, Error)]
pub enum RfnmApiError {
    #[error("The RFNM Api threw an exception: {0}")]
    ApiException(String),
    #[error("Read buffer count {0} does not match stream channel count of {1}")]
    BufferCountMismatch(usize, usize),
    #[error("Buffer sizes in stream buffers do not match. They must all be the same")]
    BufferSizeMismatch,
    #[error("Probing failed")]
    ProbeFail,
    #[error("Tuning failed")]
    TuneFail,
    #[error("Gain set failed. Make sure the value is supported")]
    GainFail,
    #[error("Timeout")]
    Timeout,
    #[error("Usb failed")]
    UsbFail,
    #[error("DqbufOverflow")]
    DqbufOverflow,
    #[error("Operation is not supported")]
    NotSupported,
    #[error("A software update is required ")]
    SoftwareUpgradeRequred,
    #[error("RFNM_API_DQBUF_OVERFLOW")]
    DqbufNoData,
    #[error("RFNM_API_DQBUF_NO_DATA")]
    MinQbufCountNotSatisfied,
    #[error("RFNM_API_MIN_QBUF_QUEUE_FULL")]
    MinQbufQueueFull,
    #[error("Encounterd an unkwon error code: {0}")]
    Unknown(u32),
}

impl From<WrappedThrownError> for RfnmApiError {
    fn from(value: WrappedThrownError) -> Self {
        let c_str = unsafe { CStr::from_ptr(value.message.as_ptr()) };
        let str = String::from_utf8_lossy(c_str.to_bytes());
        RfnmApiError::ApiException(str.to_string())
    }
}

pub fn check_code(code: rfnm_api_failcode) -> Result<(), RfnmApiError> {
    if code == rfnm_api_failcode::RFNM_API_OK {
        Ok(())
    } else {
        Err(match code {
            rfnm_api_failcode::RFNM_API_PROBE_FAIL => RfnmApiError::ProbeFail,
            rfnm_api_failcode::RFNM_API_TUNE_FAIL => RfnmApiError::TuneFail,
            rfnm_api_failcode::RFNM_API_GAIN_FAIL => RfnmApiError::GainFail,
            rfnm_api_failcode::RFNM_API_TIMEOUT => RfnmApiError::Timeout,
            rfnm_api_failcode::RFNM_API_USB_FAIL => RfnmApiError::UsbFail,
            rfnm_api_failcode::RFNM_API_DQBUF_OVERFLOW => RfnmApiError::DqbufOverflow,
            rfnm_api_failcode::RFNM_API_NOT_SUPPORTED => RfnmApiError::NotSupported,
            rfnm_api_failcode::RFNM_API_SW_UPGRADE_REQUIRED => RfnmApiError::SoftwareUpgradeRequred,
            rfnm_api_failcode::RFNM_API_DQBUF_NO_DATA => RfnmApiError::DqbufNoData,
            rfnm_api_failcode::RFNM_API_MIN_QBUF_CNT_NOT_SATIFIED => {
                RfnmApiError::MinQbufCountNotSatisfied
            }
            rfnm_api_failcode::RFNM_API_MIN_QBUF_QUEUE_FULL => RfnmApiError::MinQbufQueueFull,
            rfnm_api_failcode(code) => RfnmApiError::Unknown(code as u32),
        })
    }
}

pub fn channel_flag_to_number(channel: rfnm_channel) -> Option<u32> {
    for i in 0..7 {
        if channel.0 == (1 << i) {
            return Some(i);
        }
    }

    None
}
