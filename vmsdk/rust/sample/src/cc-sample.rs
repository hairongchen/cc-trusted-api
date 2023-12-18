use cctrusted::{dump_cc_report, get_default_algorithm, get_cc_report, ExtraArgs};

fn main() {

    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    let quote = match get_cc_report(nonce, data, ExtraArgs{}){
        Ok(q) => q,
        Err(e) => {
            println!("error getting TDX report: {:?}",e);
        }
    };

    match get_default_algorithm(){
        Ok(algo) => println!("TDX default algo: {}", algo.algo_id_str),
        Err(e) => {
            println!("error getting TDX algo: {:?}",e);
        }
    }

    dump_cc_report(quote);
}
