use cctrusted::api::*;
use cctrusted::api_data::*;
use log::*;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report
    info!("call cc trusted API [get_cc_report] to retrieve cc report!");
    let report = match get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // dump the cc report
    info!("call cc trusted API [dump_cc_report] to dump cc report!");
    dump_cc_report(&report.cc_report);

    // get cvm default algorithm
    info!("call cc trusted API [get_default_algorithm] to get TEE supported algorithm!");
    match get_default_algorithm() {
        Ok(algorithm) => {
            info!("supported algorithm: {}", algorithm.algo_id_str);
            ();
        }
        Err(e) => {
            error!("error get algorithm: {:?}", e);
            return;
        }
    };

    let tdx_quote: TdxQuote = ParsedCcReport::parse_cc_report(report.cc_report);
    info!("name = {}, var1 = {}", tdx_quote.name, tdx_quote.var1);
}
