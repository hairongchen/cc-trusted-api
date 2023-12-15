use cctrusted::{get_cc_report, ExtraArgs};
use anyhow::*; 

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    match get_cc_report(nonce, data, ExtraArgs{}){
        Ok(q) => println!("quote: {}", q.len()),
        Err(e) => {
            return Err(anyhow!(
                "[get_cc_report] error getting TDX report: {:?}",
                e
            ))
        }
    }

}
