use anyhow::*;
use log::info;
use core::result::Result::Ok;

use crate::cc_type::*;
use crate::cvm::*;
use crate::tcg::{TcgAlgorithmRegistry, TcgDigest};
use crate::tdx::common::*;
use crate::tdx::rtmr::TdxRTMR;
use std::path::Path;

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
        cc_report_raw: the raw tdx quote in byte array
        td_report_raw: the raw td report in byte array
        rtrms: array of TdxRTMR struct
*/
pub struct TdxVM {
    pub cc_type: CcType,
    pub version: TdxVersion,
    pub device_node: DeviceNode,
    pub algo_id: u8,
    pub cc_report_raw: Vec<u8>,
    pub td_report_raw: Vec<u8>,
    pub rtrms: Vec<TdxRTMR>,
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
        let algo_id = crate::tcg::TPM_ALG_SHA384;

        TdxVM {
            cc_type,
            version,
            device_node,
            algo_id,
            cc_report_raw: Vec::new(),
            td_report_raw: Vec::new(),
            rtrms: Vec::new(),
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

    let request = match self.version {
        TdxVersion::TDX_1_0 => match Tdx::prepare_tdx_1_0_report_request(report_data) {
            Err(e) => return Err(anyhow!("[get_td_report] Fail to get TDX report: {:?}", e)),
            Ok(r) => Ok(r),
        },
        TdxVersion::TDX_1_5 => match Tdx::prepare_tdx_1_5_report_request(report_data) {
            Err(e) => return Err(anyhow!("[get_td_report] Fail to get TDX report: {:?}", e)),
            Ok(r) => Ok(r),
        },
    }

    match self.version {
        TdxVersion::TDX_1_0 => {      
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
        
            Ok(request.tdreport.to_vec())},
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

// TdxVM implements the interfaces defined in CVM trait
impl CVM for TdxVM {
    // retrieve TDX quote
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error> {

        let tdreport = match self.get_td_report(nonce, data) {
            Ok(r) => r,
            Err(e) => return Err(anyhow!("[process_cc_report] error getting TD report: {:?}", e)),
        }

        let tdx_quote_request = match self.prepare_tdx_quote_request(tdreport) {
            Ok(r) => r,
            Err(e) => return Err(anyhow!("[process_cc_report] error getting TDX quote: {:?}", e)),
        }

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

        //build the operator code and apply the ioctl command
        match self.version {
            TdxVersion::TDX_1_0 => {
                ioctl_read!(
                    get_quote_1_0_ioctl,
                    b'T',
                    TdxOperation::TDX_1_0_GET_QUOTE,
                    u64
                );
                match unsafe {
                    get_quote_1_0_ioctl(device_node.as_raw_fd(), ptr::addr_of!(tdx_quote_request) as *mut u64)
                } {
                    Err(e) => {
                        return Err(anyhow!("[get_tdx_quote] Fail to get TDX quote: {:?}", e))
                    }
                    Ok(_r) => _r,
                };
            }
            TdxVersion::TDX_1_5 => {
                ioctl_read!(
                    get_quote_1_5_ioctl,
                    b'T',
                    TdxOperation::TDX_1_5_GET_QUOTE,
                    tdx_quote_req
                );
                match unsafe {
                    get_quote_1_5_ioctl(
                        device_node.as_raw_fd(),
                        ptr::addr_of!(tdx_quote_request) as *mut tdx_quote_req,
                    )
                } {
                    Err(e) => {
                        return Err(anyhow!("[get_tdx_quote] Fail to get TDX quote: {:?}", e))
                    }
                    Ok(_r) => _r,
                };
            }
        };
    
        //inspect the response and retrive quote data
        let out_len = quote_header.out_len;
        let qgs_msg_resp_size =
            unsafe { core::mem::transmute::<[u8; 4], u32>(quote_header.data_len_be_bytes) }.to_be();
    
        let qgs_msg_resp = unsafe {
            let raw_ptr = ptr::addr_of!(quote_header.data) as *mut qgs_msg_get_quote_resp;
            raw_ptr.as_mut().unwrap() as &mut qgs_msg_get_quote_resp
        };
    
        if out_len - qgs_msg_resp_size != 4 {
            return Err(anyhow!(
                "[get_tdx_quote] Fail to get TDX quote: wrong TDX quote size!"
            ));
        }
    
        if qgs_msg_resp.header.major_version != 1
            || qgs_msg_resp.header.minor_version != 0
            || qgs_msg_resp.header.msg_type != 1
            || qgs_msg_resp.header.error_code != 0
        {
            return Err(anyhow!(
                "[get_tdx_quote] Fail to get TDX quote: QGS response error!"
            ));
        }
    
        Ok(qgs_msg_resp.id_quote[0..(qgs_msg_resp.quote_size as usize)].to_vec())
    }

    // retrieve TDX RTMR
    fn process_cc_measurement(&self, _index: u8, _algo_id: u8) -> TcgDigest {
        todo!()
    }

    // retrieve TDX CCEL and IMA eventlog
    fn process_cc_eventlog(&self) -> () {
        todo!()
    }

    fn get_cc_type(&self) -> CcType {
        return self.cc_type.clone();
    }

    fn dump(&self) {
        info!("======================================");
        info!("CVM type = {}", self.cc_type.tee_type_str);
        info!(
            "CVM version = {}",
            TDX_VERSION_MAP.get(&self.version).unwrap().to_owned()
        );
        info!("======================================");
    }
}

impl TcgAlgorithmRegistry for TdxVM {
    fn get_algorithm_id(&self) -> u8 {
        self.algo_id
    }
}

impl BuildCVM for TdxVM {}
