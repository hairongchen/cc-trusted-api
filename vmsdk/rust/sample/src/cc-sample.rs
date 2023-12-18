use cctrusted::{get_default_algorithm, Algo, get_cc_report, ExtraArgs};

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

    match get_default_algorithm(){
        Ok(algo) => println!("quote length: {}", algo.algo_id_str),
        Err(e) => {
            println!("error getting TDX algo: {:?}",e);
        }
    }
}
