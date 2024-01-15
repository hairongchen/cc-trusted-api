use anyhow::*;
use core::result::Result;
use core::result::Result::Ok;

use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::tcg::{TcgDigest, ALGO_NAME_MAP};

use crate::cvm::build_cvm;
use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_base::tcg::EventLogEntry;

pub struct API {}

impl CCTrustedApi for API {
    // CCTrustedApi trait function: get report of a CVM
    fn get_cc_report(
        nonce: String,
        data: String,
        _extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {
        match build_cvm() {
            Ok(mut cvm) => {
                // call CVM trait defined methods
                cvm.dump();
                Ok(CcReport {
                    cc_report: match cvm.process_cc_report(nonce, data) {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(anyhow!("[get_cc_report] error get cc report: {:?}", e));
                        }
                    },
                    cc_type: cvm.get_cc_type().tee_type,
                })
            }
            Err(e) => return Err(anyhow!("[get_cc_report] error create cvm: {:?}", e)),
        }
    }

    // CCTrustedApi trait function: dump report of a CVM in hex and char format
    fn dump_cc_report(report: &Vec<u8>) {
        dump_data(report)
    }

    // CCTrustedApi trait function: get measurements of a CVM
    fn get_cc_measurement(_index: u8, _algo_id: u8) -> TcgDigest {
        todo!()
    }

    // CCTrustedApi trait function: get eventlogs of a CVM
    fn get_cc_eventlog(start: Option<u32>, count: Option<u32>) -> Result<Vec<EventLogEntry>, anyhow::Error> {
        match build_cvm() {
            Ok(cvm) => {
                cvm.process_cc_eventlog(start, count)
            }
            Err(e) => return Err(anyhow!("[get_cc_eventlog] error create cvm: {:?}", e)),
        }
    }

    // CCTrustedApi trait function: get default algorithm of a CVM
    fn get_default_algorithm() -> Result<Algorithm, anyhow::Error> {
        match build_cvm() {
            Ok(cvm) => {
                // call CVM trait defined methods
                let algo_id = cvm.get_algorithm_id();
                Ok(Algorithm {
                    algo_id: algo_id,
                    algo_id_str: ALGO_NAME_MAP.get(&algo_id).unwrap().to_owned(),
                })
            }
            Err(e) => {
                return Err(anyhow!(
                    "[get_default_algorithm] error get algorithm: {:?}",
                    e
                ))
            }
        }
    }
}

#[cfg(test)]
mod sdk_api_tests {
    use super::*;

    // test on cc trusted API [get_cc_eventlog]
    #[test]
    fn test_get_cc_eventlog() {
        let event_logs = match API::get_cc_eventlog(Some(1), Some(10)) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        assert_eq!(event_logs.len(), 10);
    }

    #[test]
    fn test_get_cc_eventlog_none() {
        let event_logs = match API::get_cc_eventlog(None, None) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(false, format!("{:?}", e).is_empty());
                return;
            }
        };

        assert_ne!(event_logs.len(), 0);
    }
    
    #[test]
    fn test_get_cc_eventlog_invalid_start() {
        match API::get_cc_eventlog(Some(0), None) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(false, format!("{:?}", e).is_empty());
                return;
            }
        };
    }

    #[test]
    fn test_get_cc_eventlog_invalid_count() {
        match API::get_cc_eventlog(Some(1), Some(0)) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(false, format!("{:?}", e).is_empty());
                return;
            }
        };
    }

    #[test]
    fn test_get_cc_eventlog_check_return_type() {
        let event_logs = match API::get_cc_eventlog(Some(1), Some(5)) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        for event_log in event_logs {
            match event_log {
                EventLogEntry::TcgImrEvent(tcg_imr_event) => (),
                EventLogEntry::TcgPcClientImrEvent(tcg_pc_client_imr_event) => (),
                _ => assert_eq!(true, false),
            }
        }
    }
}
