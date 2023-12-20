use std::result::Result;
use std::result::Result::Ok;
use anyhow::*;

use cctrusted_base::cc_type::CcType;
use cctrusted_base::tcg::{TcgDigest, ALGO_NAME_MAP};
use cctrusted_base::binary_blob::dump_data;

use crate::api_data::*;

// this CC API takes nonce, data and open extra argument structure as input and returns raw TEE report
pub fn get_cc_report(
    nonce: String,
    data: String,
    _extra_args: ExtraArgs,
) -> Result<Vec<u8>, anyhow::Error> {
    match CcType::build_cvm() {
        Ok(mut cvm) => {
            // call CVM trait defined methods
            cvm.dump();
            cvm.process_cc_report(nonce, data)
        },
        Err(e) => return Err(anyhow!("[get_cc_report] error get quote: {:?}", e)),
    }
}

pub fn dump_cc_report(report: Vec<u8>) -> Result<(), anyhow::Error> {
    Ok(dump_data(report))
}

// this CC API takes IMR register index and algorithm ID as input and returns the IMR data
pub fn get_cc_measurement(_index: u8, _algo_id: u8) -> Vec<TcgDigest> {
    todo!()
}

// this CC API takes eventlog start offset and count as input and returns the eventlog data
pub fn get_cc_eventlog(_start: u16, _count: u16) -> () {
    todo!()
}

pub fn get_default_algorithm() -> Result<Algo, anyhow::Error>{
    match CcType::build_cvm() {
        Ok(cvm) => {
            // call CVM trait defined methods
            let algo_id = cvm.get_algorithm_id();
            Ok(Algo{
                algo_id: algo_id,
                algo_id_str: ALGO_NAME_MAP.get(&algo_id).unwrap().to_owned()
            })
        },
        Err(e) => return Err(anyhow!("[get_default_algorithm] error get algorithm: {:?}", e)),
    }
}

