use cctrusted::{dump_cc_report, get_cc_report, get_default_algorithm};
use cctrusted::api_data::*;

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    match get_default_algorithm() {
        Ok(algo) => println!("TDX default algo: {}", algo.algo_id_str),
        Err(e) => {
            println!("error getting TDX algo: {:?}", e);
        }
    }

    // retrieve cc report
    let quote = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            panic!("error getting TDX report: {:?}", e);
        }
    };

    // dump the cc report
    dump_cc_report(quote);
}
