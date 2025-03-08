use crate::channel_settings::{RxChannelInfo, RxChannelSettings};
use crate::{RfnmApiError, channel_flag_to_number, check_code};
use rfnm_sys::{
    DeviceWrapper,
    WrappedThrownError,
    device_connect_usb,
    device_free,
    device_get_rx_channel,
    device_get_rx_channel_count,
    device_rx_work_stop,
    device_set_rx_channel_active,
    device_tx_work_stop,
    rfnm_api_rx_ch,
    rfnm_ch_enable,
    rfnm_ch_stream,
    rfnm_channel,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Device {
    device_wrapper: *mut DeviceWrapper,
}

impl Device {
    pub fn connect_usb() -> Result<Self, RfnmApiError> {
        let mut throw_error = WrappedThrownError::empty();
        let device_wrapper = unsafe { device_connect_usb(&mut throw_error) };
        if device_wrapper.is_null() {
            Err(throw_error.into())
        } else {
            // things might be weird, for example due to ungraceful shutdowns
            // it is not fine if this fails by the way
            unsafe {
                check_code(device_rx_work_stop(device_wrapper))?;
                check_code(device_tx_work_stop(device_wrapper))?;
                // this uses the defaults from the api itself so it should be a somewhat sane state afterwards.
                for i in 0..device_get_rx_channel_count(device_wrapper) {
                    let mut channel_info = rfnm_api_rx_ch::default();
                    check_code(device_get_rx_channel(device_wrapper, i, &mut channel_info))?;
                    let valid_center =
                        channel_info.freq_min + (channel_info.freq_max - channel_info.freq_min) / 2;
                    let settings = RxChannelSettings {
                        frequency: valid_center,
                        ..Default::default()
                    };
                    settings.apply_to_device(device_wrapper, i)?;
                    check_code(device_set_rx_channel_active(
                        device_wrapper,
                        i,
                        rfnm_ch_enable::RFNM_CH_OFF,
                        rfnm_ch_stream::RFNM_CH_STREAM_OFF,
                        true,
                    ))?;
                }
                //for i in 0..device_get_tx_channel_count(device_wrapper) {
                //check_code(device_set_tx_channel_active(device_wrapper,i,rfnm_ch_enable::RFNM_CH_OFF, rfnm_ch_stream::RFNM_CH_STREAM_OFF, true))?;
                //}
            }
            Ok(Self { device_wrapper })
        }
    }

    pub(crate) fn wrapper(&self) -> *mut DeviceWrapper {
        self.device_wrapper
    }

    pub fn set_rx_settings(
        &self,
        channel: rfnm_channel,
        settings: &RxChannelSettings,
    ) -> Result<(), RfnmApiError> {
        unsafe {
            settings.apply_to_device(
                self.device_wrapper,
                channel_flag_to_number(channel).unwrap_or(0),
            )
        }
    }

    pub fn get_rx_settings(&self, channel: rfnm_channel) -> Result<RxChannelInfo, RfnmApiError> {
        unsafe {
            RxChannelInfo::from_device(
                self.device_wrapper,
                channel_flag_to_number(channel).unwrap_or(0),
            )
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { device_free(self.device_wrapper) };
    }
}

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Initialization has failed.")]
    InitFailed,
}
