use crate::{check_code, RfnmApiError};
use rfnm_sys::{
    device_set_rx_channel_freq, device_set_rx_channel_gain, device_set_rx_channel_samp_freq_div,
    DeviceWrapper,
};

pub struct RxSettings {
    pub frequency: i64,
    pub gain: i8,
    pub rate_divider_settings: SampleRateDividerSettings,
}

impl Default for RxSettings {
    fn default() -> Self {
        Self {
            frequency: 100_000_000,
            gain: 0,
            rate_divider_settings: Default::default(),
        }
    }
}

impl RxSettings {
    pub(crate) unsafe fn apply_to_device(
        &self,
        wrapper: *mut DeviceWrapper,
        channel_num: u32,
    ) -> Result<(), RfnmApiError> {
        check_code(device_set_rx_channel_samp_freq_div(
            wrapper,
            channel_num,
            self.rate_divider_settings.m,
            self.rate_divider_settings.m,
            false,
        ))?;
        check_code(device_set_rx_channel_gain(
            wrapper,
            channel_num,
            self.gain,
            false,
        ))?;
        check_code(device_set_rx_channel_freq(
            wrapper,
            channel_num,
            self.frequency,
            true,
        ))?;
        Ok(())
    }
}

pub struct SampleRateDividerSettings {
    pub m: i16,
    pub n: i16,
}

impl Default for SampleRateDividerSettings {
    fn default() -> Self {
        Self { m: 1, n: 1 }
    }
}
