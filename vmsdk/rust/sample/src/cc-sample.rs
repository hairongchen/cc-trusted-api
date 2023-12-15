use cctrusted::{get_cc_report, ExtraArgs};

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    let q = get_cc_report(nonce, data, ExtraArgs{});

    println!("quote: {}", q);

}
