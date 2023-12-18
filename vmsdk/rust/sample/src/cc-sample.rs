use cctrusted::{get_cc_report, ExtraArgs};

fn main() {

    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    match get_cc_report(nonce, data, ExtraArgs{}){
        Ok(q) => println!("quote length: {}", q.len()),
        Err(e) => {
            println!("error getting TDX report: {:?}",e);
        }
    }

    let default_algo =  get_default_algorithms();
    println!("default algo is: {}", default_algo.default_algo_str);

}
