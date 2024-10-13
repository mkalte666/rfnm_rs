use std::ffi::{CStr};
use std::mem::MaybeUninit;
use rfnm_sys::{rfnm_dev_hwinfo, rfnm_dev_hwinfo_bit};

#[derive(Debug,Clone)]
pub struct HwInfo {
    pub protocol_version : u32,
    pub motherboard: BoardInfo,
    pub daughterboards: [Option<BoardInfo>; 2],
    pub clock_info: ClockInfo,
}

#[derive(Debug,Clone)]
pub struct BoardInfo {
    pub id: u8,
    pub revision : u8,
    pub serial: [u8;9],
    pub name: String,
    pub mac_addr: Option<[u8;6]>,
    pub channel_counts: ChannelCounts,
}


impl From<rfnm_dev_hwinfo> for HwInfo {
    fn from(value: rfnm_dev_hwinfo) -> Self {
        let db1 = if value.daughterboard[0].board_id == 0 {
            None
        } else {
            Some(value.daughterboard[0].into())
        };
        let db2 = if value.daughterboard[1].board_id == 0 {
            None
        } else {
            Some(value.daughterboard[1].into())
        };
        HwInfo {
            protocol_version: value.protocol_version,
            motherboard: value.motherboard.into(),
            daughterboards: [db1,db2],
            clock_info: ClockInfo { dcs_clk: value.clock.dcs_clk },
        }
    }
}

impl From<rfnm_dev_hwinfo_bit> for BoardInfo {
    fn from(value: rfnm_dev_hwinfo_bit) -> Self {
        let name_cstr = unsafe {CStr::from_ptr(value.user_readable_name.as_ptr())};
        let name = String::from_utf8_lossy(name_cstr.to_bytes()).to_string();
        Self {
            id: value.board_id,
            revision: value.board_revision_id,
            serial: value.serial_number,
            name,
            mac_addr: Some(value.mac_addr),
            channel_counts: ChannelCounts {
                rx: value.rx_ch_cnt,
                tx: value.tx_ch_cnt
            },
        }
    }
}

#[derive(Debug,Clone)]
pub struct ClockInfo {
    pub dcs_clk: u64
}

#[derive(Debug,Clone)]
pub struct ChannelCounts {
    pub rx: u8,
    pub tx: u8
}


/// Discover all connected rfnm devices.
pub fn discover_usb_boards() -> Vec<HwInfo>
{
    let mut dst = Vec::new();
    unsafe {
        let board_count = rfnm_sys::find_usb_devices(std::ptr::null_mut(),0);
        let mut dst_vec = vec![MaybeUninit::uninit();board_count];
        let actual_count = rfnm_sys::find_usb_devices(dst_vec.as_mut_ptr() as *mut rfnm_dev_hwinfo, dst_vec.len());
        for i in 0..actual_count {
            let raw_hw_info : rfnm_dev_hwinfo = dst_vec[i].assume_init();
            dst.push(raw_hw_info.into())
        }
    }

    dst
}