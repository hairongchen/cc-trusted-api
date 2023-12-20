use log::*;
use cctrusted::api_data::*;
use cctrusted::api::{dump_cc_report, get_cc_report};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    let quote = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            err!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // dump the cc report
    match dump_cc_report(quote) {
        Ok(_) => (),
        Err(e) => {
            err!("error dump quote: {:?}", e);
            return;
        }
    };
}
