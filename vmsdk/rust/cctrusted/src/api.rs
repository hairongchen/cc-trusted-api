use std::result::Result;
use std::result::Result::Ok;
use anyhow::*;

use cctrusted_base::cc_type::CcType;
use cctrusted_base::tcg::TcgDigest;

use crate::api_data::*;

// this CC API takes nonce, data and open extra argument structure as input and returns raw TEE report
pub fn get_cc_report(
    nonce: String,
    data: String,
    _extra_args: ExtraArgs,
) -> Result<Vec<u8>, anyhow::Error> {
     let mut cvm = match CcType::build_cvm() {
        Ok(c) => c,
        Err(e) => return Err(anyhow!("[get_cc_report] error getting quote: {:?}", e)),
    };

    // call CVM trait defined methods
    cvm.dump();
    cvm.process_cc_report(nonce, data)
}

pub fn dump_cc_report(report: Vec<u8>) -> Result {
    let cvm = match CcType::build_cvm() {
        Ok(c) => c,
        Err(e) => return Err(anyhow!("[get_cc_report] error getting quote: {:?}", e)),
    };

    cvm.dump_cc_report(report);

    Ok()
}

// this CC API takes IMR register index and algorithm ID as input and returns the IMR data
pub fn get_cc_measurement(_index: u8, _algo_id: u8) -> Vec<TcgDigest> {
    todo!()
}

// this CC API takes eventlog start offset and count as input and returns the eventlog data
pub fn get_cc_eventlog(_start: u16, _count: u16) -> () {
    todo!()
}

