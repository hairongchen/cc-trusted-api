use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_vm::sdk::API;
use log::*;

fn main() {
    // set log level
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let nonce = "MTIzNDU2Nzg=".to_string();
    let data = "YWJjZGVmZw==".to_string();

    // retrieve cc report with API "get_cc_report"
    info!("call cc trusted API [get_cc_report] to retrieve cc report!");
    let _report = match API::get_cc_report(nonce, data, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting TDX report: {:?}", e);
            return;
        }
    };

    // retrieve cc eventlog with API "get_cc_eventlog"    
    let event_logs = match API::get_cc_eventlog(Some(1),Some(10)) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting TDX report: {:?}", e);
            return;
        }
    };

    info!("event log count: {}", event_logs.len());
    // for event_log in event_logs {
    //     event_log.show();
    // }

}
