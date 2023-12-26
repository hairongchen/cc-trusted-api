#![allow(non_camel_case_types)]

use anyhow::*;
use log::info;
use core::convert::TryInto;
use core::mem;
use core::ptr;
use core::result::Result::Ok;
use core::result::Result;
use cctrusted_base::cc_type::*;
use crate::cvm::*;
use cctrusted_base::tcg::{TcgAlgorithmRegistry, TcgDigest};
use cctrusted_base::tdx::common::*;
use std::path::Path;
use nix::*;
use cctrusted_base::tdx::report::*;
use cctrusted_base::tdx::quote::*;
use std::fs::File;
use std::os::fd::AsRawFd;


// TDX ioctl operation code to be used for get TDX quote and TD Report
pub enum TdxOperation {
    TDX_GET_TD_REPORT = 1,
    TDX_1_0_GET_QUOTE = 2,
    TDX_1_5_GET_QUOTE = 4,
}

/*
    TdxVM is an abstraction of TDX running environment, it contains:
        cc_type: should always be CcType built with TeeType::TDX
        version: TdxVersion::TDX_1_0 or TdxVersion::TDX_1_5
        device_node: /dev/tdx-guest or /dev/tdx_guest
        algo_id: should be TPM_ALG_SHA384
*/
pub struct TdxVM {
    pub cc_type: CcType,
    pub version: TdxVersion,
    pub device_node: DeviceNode,
    pub algo_id: u8,
}

// implement the structure method and associated function
impl TdxVM {
    // associated function: to build a TdxVM sturcture instance
    pub fn new() -> TdxVM {
        let cc_type = CcType {
            tee_type: TeeType::TDX,
            tee_type_str: TEE_NAME_MAP.get(&TeeType::TDX).unwrap().to_owned(),
        };

        let version = Self::get_tdx_version();
        let device_node = DeviceNode {
            device_path: TDX_DEVICE_NODE_MAP.get(&version).unwrap().to_owned(),
        };
        let algo_id = cctrusted_base::tcg::TPM_ALG_SHA384;

        TdxVM {
            cc_type,
            version,
            device_node,
            algo_id,
        }
    }

    pub fn get_td_report(&self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error> {

        let report_data = match Tdx::generate_tdx_report_data(nonce, Some(data)) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_td_report] error generating TDX report data: {:?}",
                    e
                ))
            }
        };

        let device_node = match File::options()
        .read(true)
        .write(true)
        .open(self.device_node.device_path.clone())
        {
            Err(e) => {
                return Err(anyhow!(
                    "[get_td_report] Fail to open {}: {:?}",
                    self.device_node.device_path,
                    e
                ))
            }
            Ok(fd) => fd,
        };

        match self.version {
            TdxVersion::TDX_1_0 => {
                    let report_data_bytes = match base64::decode(report_data) {
                        Ok(v) => v,
                        Err(e) => return Err(anyhow!("report data is not base64 encoded: {:?}", e)),
                    };
                
                    //prepare get TDX report request data
                    let mut report_data_array: [u8; REPORT_DATA_LEN as usize] = [0; REPORT_DATA_LEN as usize];
                    report_data_array.copy_from_slice(&report_data_bytes[0..]);
                    let td_report: [u8; TDX_REPORT_LEN as usize] = [0; TDX_REPORT_LEN as usize];
                
                    //build the request
                    let request = tdx_1_0_report_req {
                        subtype: 0 as u8,
                        reportdata: ptr::addr_of!(report_data_array) as u64,
                        rpd_len: REPORT_DATA_LEN,
                        tdreport: ptr::addr_of!(td_report) as u64,
                        tdr_len: TDX_REPORT_LEN,
                    };

                    //build the operator code
                    ioctl_readwrite!(
                        get_report_1_0_ioctl,
                        b'T',
                        TdxOperation::TDX_GET_TD_REPORT,
                        u64
                    );
                
                    //apply the ioctl command
                    match unsafe {
                        get_report_1_0_ioctl(device_node.as_raw_fd(), ptr::addr_of!(request) as *mut u64)
                    } {
                        Err(e) => {
                            return Err(anyhow!(
                                "[get_td_report] Fail to get TDX report: {:?}",
                                e
                            ))
                        }
                        Ok(_) => (),
                    };
                
                    Ok(td_report.to_vec())
            },
            TdxVersion::TDX_1_5 => {
                    let report_data_bytes = match base64::decode(report_data) {
                        Ok(v) => v,
                        Err(e) => return Err(anyhow!("report data is not base64 encoded: {:?}", e)),
                    };
                
                    //prepare get TDX report request data
                    let mut request = tdx_1_5_report_req {
                        reportdata: [0; REPORT_DATA_LEN as usize],
                        tdreport: [0; TDX_REPORT_LEN as usize],
                    };
                    request.reportdata.copy_from_slice(&report_data_bytes[0..]);

                    //build the operator code
                    ioctl_readwrite!(
                        get_report_1_5_ioctl,
                        b'T',
                        TdxOperation::TDX_GET_TD_REPORT,
                        tdx_1_5_report_req
                    );
                
                    //apply the ioctl command
                    match unsafe {
                        get_report_1_5_ioctl(
                            device_node.as_raw_fd(),
                            ptr::addr_of!(request) as *mut tdx_1_5_report_req,
                        )
                    } {
                        Err(e) => {
                            return Err(anyhow!(
                                "[get_td_report] Fail to get TDX report: {:?}",
                                e
                            ))
                        }
                        Ok(_) => (),
                    };
                
                    Ok(request.tdreport.to_vec())
            },
        }
    }

    // associated function to detect the TDX version
    fn get_tdx_version() -> TdxVersion {
        if Path::new(TEE_TDX_1_0_PATH).exists() {
            TdxVersion::TDX_1_0
        } else if Path::new(TEE_TDX_1_5_PATH).exists() {
            TdxVersion::TDX_1_5
        } else {
            TdxVersion::TDX_1_0
        }
    }
}

impl TcgAlgorithmRegistry for TdxVM {
    fn get_algorithm_id(&self) -> u8 {
        self.algo_id
    }
}

impl BuildCVM for TdxVM {}
