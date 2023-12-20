use log::*;
use cctrusted::api_data::*;
use cctrusted::api::{dump_cc_report, get_cc_report};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    info!("call cc trusted API [get_cc_report] to retrieve cc report!");
    let quote = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // dump the cc report
    info!("call cc trusted API [dump_cc_report] to dump cc report!");
    match dump_cc_report(quote) {
        Ok(_) => (),
        Err(e) => {
            error!("error dump quote: {:?}", e);
            return;
        }
    };
}
