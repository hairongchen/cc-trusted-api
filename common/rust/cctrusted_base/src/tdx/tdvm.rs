use crate::cc_type::*;
use crate::cvm::*;
use crate::tdx::common::*;
use crate::tdx::report::generate_tdx_report_data;

use anyhow::*; 
use std::result::Result::Ok;

struct TdReport {}

struct TdxQuote {}
/*
    TdxVM is an abstraction of TDX running environment, it contains:
        cc_type: should always be TDX
        version: 1.0 or 1.5
        device_node: /dev/tdx-guest or /dev/tdx_guest
        algo_id: should be TPM_ALG_SHA384
        cc_report_raw: the raw tdx quote in byte vector, filled by get_cc_report()
        //cc_report: the parsed tdx quote, filled by get_cc_report()
        td_report_raw: the raw td report in byte vector, filled by get_cc_report()
        //td_report: the parsed tdreport, filled by get_cc_report()
        rtrms: TDX rtmr algorithm and hash, filled by get_cc_measurement()
*/
pub struct TdxVM {
    pub cc_type: CcType,
    pub version: TdxVersion,
    pub device_node: DeviceNode,
    pub algo_id: u8,
    pub cc_report_raw: Vec<u8>,
    //cc_report: Option<CcReport>,
    pub td_report_raw: Vec<u8>,
    //td_report: Option<TdReport>,
    pub rtrms: Vec<TdxRTMR>,
}

// implement the structure create function
impl TdxVM {
    pub fn new() -> TdxVM {
        let cc_type = CcType{tee_type: TeeType::TDX, tee_type_str: TeeNameMap.get(&TeeType::TDX)};

        let version = get_tdx_version();
        let device_node = DeviceNode {device_path: TdxDeviceNodeMap.get(&version)};
        let algo_id = tcg::TPM_ALG_SHA384;

        TdxVM {
            cc_type,
            version,
            device_node,
            algo_id,
            cc_report_raw: None,
            td_report_raw: None,
            rtrms: None
        }
    }
}

// all TdxVM's interfaces should implement CVM trait
impl CVM for TdxVM {
    // retrieve TDX quote
    fn process_cc_report(&self, nonce: String, data: String) -> Result<Vec<u8>, anyhow::Error>{
        let report_data = match generate_tdx_report_data(nonce, Some(data)) {
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!(
                    "[get_cc_report] error generating TDX report data: {:?}",
                    e
                ))
            }
        };

        self.cc_report_raw = match get_tdx_quote(&self, report_data) {
            Ok(q) => Some(q),
            Err(e) => return Err(anyhow!(
                "[get_cc_report] error getting TDX quote: {:?}",
                e
            )),
        };

        return Ok(self.cc_report_raw)
    }

    // retrieve TDX RTMR
    fn process_cc_measurement() -> () {
        todo!()
    }

    // retrieve TDX CCEL and IMA eventlog
    fn process_cc_eventlog() -> () {
        todo!()
    }

}