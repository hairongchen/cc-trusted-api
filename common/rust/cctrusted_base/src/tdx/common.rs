#![allow(non_camel_case_types)]
use std::collections::HashMap;
use crate::cc_type::*;
use std::path::Path;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum TdxVersion {
    TDX_1_0,
    TDX_1_5,
}

lazy_static! {
    pub static  ref TdxVersionMap: HashMap<TdxVersion, String> = {
        let mut map:HashMap<TdxVersion, String> = HashMap::new();
        map.insert(TdxVersion::TDX_1_0, "1.0".to_string());
        map.insert(TdxVersion::TDX_1_5, "1.5".to_string());
        map
    };
}

lazy_static! {
    pub static  ref TdxDeviceNodeMap: HashMap<TdxVersion, String> = {
        let mut map:HashMap<TdxVersion, String> = HashMap::new();
        map.insert(TdxVersion::TDX_1_0, TEE_TDX_1_0_PATH.to_string());
        map.insert(TdxVersion::TDX_1_5, TEE_TDX_1_5_PATH.to_string());
        map
    };
}


pub enum TdxOperation {
    TDX_GET_TD_REPORT = 1,
    TDX_1_0_GET_QUOTE = 2,
    TDX_1_5_GET_QUOTE = 4,
}

pub const REPORT_DATA_LEN: u32 = 64;
pub const TDX_REPORT_LEN: u32 = 1024;
pub const TDX_QUOTE_LEN: usize = 4 * 4096;

pub fn get_tdx_version() -> TdxVersion {
    if Path::new(TEE_TDX_1_0_PATH).exists() {
        TdxVersion::TDX_1_0
    } else if Path::new(TEE_TDX_1_5_PATH).exists() {
        TdxVersion::TDX_1_5
    } else {
        panic!("get_tdx_version: no TDX device found!");
    }
}
