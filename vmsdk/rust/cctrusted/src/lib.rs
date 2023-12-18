mod api

pub fn get_cc_report(nonce: String, data: String, _extra_args: ExtraArgs) -> Result<Vec<u8>, anyhow::Error> {}
pub fn get_cc_measurement(_index: u8, _algo_id: u8) -> Vec<TcgDigest> {}
pub fn get_cc_eventlog(_start: u16, _count: u16) -> () {}
