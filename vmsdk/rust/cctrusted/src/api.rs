use anyhow::*;

struct ExtraArgs {}

// this CC API takes nonce, data and open extra argument structure as input and returns raw TEE report
pub fn get_cc_report(nonce: String, data: String, extraArgs: ExtraArgs) -> Vec<u8> {
    todo!()
}

// this CC API takes IMR register index and algorithm ID as input and returns the IMR data
pub fn get_cc_measurement(index: u8, algo_id: u8) -> Vec!<TcgDigest> {
    todo!()
}

// this CC API takes eventlog start and count as input and returns the eventlog data
pub fn get_cc_eventlog(start: u16, count: u16) -> () {
    todo!()
}