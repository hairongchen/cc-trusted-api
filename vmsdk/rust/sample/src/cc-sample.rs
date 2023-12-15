use mod cctrusted::{get_cc_report, ExtraArgs};

fn main() {
    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    get_cc_report(nonce, data, ExtraArgs{});

    println!("OK!");

}
