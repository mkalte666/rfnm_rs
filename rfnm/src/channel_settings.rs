use crate::{RfnmApiError, check_code};
use rfnm_sys::{
    DeviceWrapper,
    device_get_rx_channel,
    device_set_rx_channel_freq,
    device_set_rx_channel_gain,
    device_set_rx_channel_path,
    device_set_rx_channel_samp_freq_div,
    rfnm_api_rx_ch,
    rfnm_rf_path,
};
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;

/// This struct represents the full range of possible *everything* a rx channel can be, as well as its current state.
/// Only a subset of this can actually be set during runtime.
/// That subset is encoded in `crate::RxChannelSettings`.
#[derive(Debug, Clone)]
pub struct RxChannelInfo {
    raw: rfnm_api_rx_ch,
}

impl RxChannelInfo {
    /// Extract the part of the `RxChannelInfo` that can be updated at runtime.
    pub fn to_settings(&self) -> RxChannelSettings {
        RxChannelSettings {
            frequency: self.raw.freq,
            gain: self.raw.gain,
            rate_divider_settings: SampleRateDividerSettings {
                m: self.raw.samp_freq_div_m,
                n: self.raw.samp_freq_div_n,
            },
            path: self.path(),
        }
    }

    pub(crate) unsafe fn from_device(
        wrapper: *mut DeviceWrapper,
        channel_num: u32,
    ) -> Result<Self, RfnmApiError> {
        let mut raw: MaybeUninit<rfnm_api_rx_ch> = MaybeUninit::uninit();
        let raw = unsafe {
            check_code(device_get_rx_channel(
                wrapper,
                channel_num,
                raw.as_mut_ptr(),
            ))?;
            raw.assume_init()
        };

        Ok(Self { raw })
    }

    pub fn freq(&self) -> i64 {
        self.raw.freq
    }

    pub fn available_paths(&self) -> impl IntoIterator<Item = RfPath> {
        let paths = self.raw.path_possible;
        paths.into_iter().filter_map(|raw_path| {
            if raw_path != rfnm_rf_path::RFNM_PATH_NULL {
                Some(RfPath(raw_path))
            } else {
                None
            }
        })
    }

    pub fn path(&self) -> RfPath {
        RfPath(self.raw.path)
    }

    pub fn preferred_path(&self) -> RfPath {
        RfPath(self.raw.path_preferred)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleRateDividerSettings {
    pub m: i16,
    pub n: i16,
}

impl Default for SampleRateDividerSettings {
    fn default() -> Self {
        Self { m: 1, n: 1 }
    }
}

/// The settable portion of the RxChannelInfo. All members are public to ease editing.
pub struct RxChannelSettings {
    pub frequency: i64,
    pub gain: i8,
    pub rate_divider_settings: SampleRateDividerSettings,
    pub path: RfPath,
}

impl Default for RxChannelSettings {
    fn default() -> Self {
        Self {
            frequency: 100_000_000,
            gain: 0,
            rate_divider_settings: Default::default(),
            path: RfPath::default(),
        }
    }
}

impl RxChannelSettings {
    pub(crate) unsafe fn apply_to_device(
        &self,
        wrapper: *mut DeviceWrapper,
        channel_num: u32,
    ) -> Result<(), RfnmApiError> {
        unsafe {
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
            check_code(device_set_rx_channel_path(
                wrapper,
                channel_num,
                self.path.0,
                false,
            ))?;
            // Do frequency last, as it is the most likely to affect things across the board.
            check_code(device_set_rx_channel_freq(
                wrapper,
                channel_num,
                self.frequency,
                true,
            ))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RfPath(pub rfnm_rf_path);

impl Display for RfPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            rfnm_rf_path::RFNM_PATH_EMBED_ANT => write!(f, "embed"),
            rfnm_rf_path::RFNM_PATH_LOOPBACK => write!(f, "loopback"),
            rfnm_rf_path::RFNM_PATH_NULL => write!(f, "null"),
            p if p.0 < 8 => {
                let letter = char::from_u32('A' as u32 + p.0 as u32).unwrap_or('?');
                write!(f, "SMA_{letter}")
            }
            p => write!(f, "Invalid channel (saw id: {})", p.0),
        }
    }
}

impl From<rfnm_rf_path> for RfPath {
    fn from(value: rfnm_rf_path) -> Self {
        Self(value)
    }
}

impl Default for RfPath {
    fn default() -> Self {
        Self(rfnm_rf_path::RFNM_PATH_SMA_A)
    }
}
