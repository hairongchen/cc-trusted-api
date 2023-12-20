use cctrusted::api::{dump_cc_report, get_cc_report, get_default_algorithm};
use cctrusted::api_data::*;
use log::*;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    info!("call cc trusted API [get_cc_report] to retrieve cc report!");
    let cc_report = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // dump the cc report
    info!("call cc trusted API [dump_cc_report] to dump cc report!");
    match dump_cc_report(cc_report) {
        Ok(_) => (),
        Err(e) => {
            error!("error dump cc report: {:?}", e);
            return;
        }
    };

    // get cvm default algorithm
    info!("call cc trusted API [get_default_algorithm] to get TEE supported algorithm!");
    match get_default_algorithm() {
        Ok(algo) => {
            info!("supported algorithm: {}", algo.algo_id_str);
            ();
        }
        Err(e) => {
            error!("error get algorithm: {:?}", e);
            return;
        }
    };
}
