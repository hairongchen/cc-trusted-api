use anyhow::*;
use cctrusted_base::tdx::tdvm::TdxVM;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::cc_type::{detect_cc_type, TeeType};
use std::result::Result;

struct ExtraArgs {}

// this CC API takes nonce, data and open extra argument structure as input and returns raw TEE report
pub fn get_cc_report(nonce: String, data: String, extraArgs: ExtraArgs) -> Result<Vec<u8>, anyhow::Error> {

    // instance a cvm according to TEE detection result
    let cvm = match detect_cc_type(){
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

// this CC API takes IMR register index and algorithm ID as input and returns the IMR data
pub fn get_cc_measurement(index: u8, algo_id: u8) -> Vec<TcgDigest> {
    todo!()
}

// this CC API takes eventlog start and count as input and returns the eventlog data
pub fn get_cc_eventlog(start: u16, count: u16) -> () {
    todo!()
}