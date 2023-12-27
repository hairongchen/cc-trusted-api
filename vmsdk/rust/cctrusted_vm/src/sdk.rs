use anyhow::*;
use core::result::Result;
use core::result::Result::Ok;

use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::eventlog::TcgEventLog;
use cctrusted_base::tcg::{TcgDigest, ALGO_NAME_MAP};

use cctrusted_base::api_data::*;
use cctrusted_base::api::*;
use crate::cvm::build_cvm;

pub struct API {}

impl CCTrustedApi for API {

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
    
    fn dump_cc_report(report: &Vec<u8>) {
        dump_data(report)
    }
    
    fn get_cc_measurement(_index: u8, _algo_id: u8) -> TcgDigest {
        todo!()
    }
    
    fn get_cc_eventlog(_start: u16, _count: u16) -> TcgEventLog {
        todo!()
    }

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


