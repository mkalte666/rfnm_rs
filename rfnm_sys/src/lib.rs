#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/librfnm_wrap.rs"));

impl WrappedThrownError {
    pub fn empty() -> Self {
        Self { message: [0; 256] }
    }
}
