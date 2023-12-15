use cctrusted::{get_cc_report, ExtraArgs};

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    match get_cc_report(nonce, data, ExtraArgs{}){
        Ok(q) => println!("quote: {}", q.len()),
        Err(e) => {
            println!("[get_cc_report] error getting TDX report: {:?}",e);
        }
    }

}
