use anyhow::*;
use std::result::Result::Ok;

use crate::cc_type::*;
use crate::cvm::*;
use crate::tcg::{TcgAlgorithmRegistry, ALGO_NAME_MAP};
use crate::tdx::common::*;
use crate::tdx::rtmr::TdxRTMR;
use std::path::Path;

/*
    TdxVM is an abstraction of TDX running environment, it contains:
        cc_type: should always be TDX
        version: 1.0 or 1.5
        device_node: /dev/tdx-guest or /dev/tdx_guest
        algo_id: should be TPM_ALG_SHA384
        cc_report_raw: the raw tdx quote in byte vector, filled by get_cc_report()
        td_report_raw: the raw td report in byte vector, filled by get_cc_report()
        rtrms: TDX rtmr algorithm and hash, filled by get_cc_measurement()
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

// implement the structure create function
impl TdxVM {
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

    // function to detect the TDX version
    fn get_tdx_version() -> TdxVersion {
        if Path::new(TEE_TDX_1_0_PATH).exists() {
            TdxVersion::TDX_1_0
        } else if Path::new(TEE_TDX_1_5_PATH).exists() {
            TdxVersion::TDX_1_5
        } else {
            panic!("get_tdx_version: no TDX device found!");
        }
    }
}

// all TdxVM's interfaces should implement CVM trait
impl CVM for TdxVM {
    // retrieve TDX quote
    fn process_cc_report(&mut self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error> {
        let report_data = match self.generate_tdx_report_data(nonce, Some(data)) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_cc_report] error generating TDX report data: {:?}",
                    e
                ))
            }
        };

        match self.get_tdx_quote(report_data) {
            Ok(q) => Ok(q),
            Err(e) => return Err(anyhow!("[get_cc_report] error getting TDX quote: {:?}", e)),
        }

    }

    // retrieve TDX RTMR
    fn process_cc_measurement(&self) -> () {
        todo!()
    }

    // retrieve TDX CCEL and IMA eventlog
    fn process_cc_eventlog(&self) -> () {
        todo!()
    }

    fn parse_cc_report(&self) {
        todo!()
    }

    fn parse_cc_measurement(&self) {
        todo!()
    }
    fn parse_cc_eventlog(&self) {
        todo!()
    }

    fn dump(&self) {
        println!("======================================");
        println!("CVM type = {}", self.cc_type.tee_type_str);
        println!(
            "CVM version = {}",
            TDX_VERSION_MAP.get(&self.version).unwrap().to_owned()
        );
        println!("======================================");
    }

    fn dump_cc_measurement(&self) {
        todo!()
    }
    fn dump_cc_eventlog(&self) {
        todo!()
    }
}

impl TcgAlgorithmRegistry for TdxVM {
    fn get_algorithm_string(&self) -> String {
        ALGO_NAME_MAP.get(&self.algo_id).unwrap().to_owned()
    }
}
