use cctrusted::api_data::*;
use cctrusted::api::{dump_cc_report, get_cc_report};

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    let quote = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            println!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // dump the cc report
    match dump_cc_report(quote) {
        Ok(_) => (),
        Err(e) => println!("error dump quote: {:?}", e),
    };
}
