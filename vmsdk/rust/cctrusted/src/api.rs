use anyhow::*;
use cctrusted_base::tdx::tdvm::TdxVM;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::cc_type::{detect_cc_type, TeeType};
use std::result::Result;
use std::result::Result::Ok;
use cctrusted_base::tcg::TcgAlgorithmRegistry;
use cctrusted_base::cvm::CVM;

// this struct is used in vTPM and other TEE scenarios
// e.g.: vTPM may need report based on selective PCRs
pub struct ExtraArgs {}

// return structure for get_default_algorithm
pub struct Algo {
    pub algo_id: u8,
    pub algo_id_str: String
}

pub fn get_default_algorithm() -> Result<Algo, anyhow::Error> {
    // instance a cvm according to detected TEE type
    let cvm = match detect_cc_type().tee_type {
        TeeType::TDX => {
            TdxVM::new()
        },
        TeeType::SEV => todo!(),
        TeeType::CCA => todo!(),
        TeeType::TPM => todo!(),
        TeeType::PLAIN => return Err(anyhow!("[get_cc_report] Error: not in any TEE!")),
    };

    Ok(Algo{
        algo_id: cvm.algo_id,
        algo_id_str: cvm.get_algorithm_string()
    })
}

// this CC API takes nonce, data and open extra argument structure as input and returns raw TEE report
pub fn get_cc_report(nonce: String, data: String, _extra_args: ExtraArgs) -> Result<Vec<u8>, anyhow::Error> {

    // instance a cvm according to detected TEE type
    let mut cvm = match detect_cc_type().tee_type {
        TeeType::TDX => {
            TdxVM::new()
        },
        TeeType::SEV => todo!(),
        TeeType::CCA => todo!(),
        TeeType::TPM => todo!(),
        TeeType::PLAIN => return Err(anyhow!("[get_cc_report] Error: not in any TEE!")),
    };

    // call CVM trait defined methods
    cvm.dump();
    cvm.process_cc_report(nonce, data)
}

pub fn dump_cc_report(report: Vec<u8>) {
    
    match detect_cc_type().tee_type {
        TeeType::TDX => {
            TdxVM::dump_cc_report(report);
        },
        TeeType::SEV => todo!(),
        TeeType::CCA => todo!(),
        TeeType::TPM => todo!(),
        TeeType::PLAIN => todo!(),
    };
}

// this CC API takes IMR register index and algorithm ID as input and returns the IMR data
pub fn get_cc_measurement(_index: u8, _algo_id: u8) -> Vec<TcgDigest> {
    todo!()
}

// this CC API takes eventlog start offset and count as input and returns the eventlog data
pub fn get_cc_eventlog(_start: u16, _count: u16) -> () {
    todo!()
}
