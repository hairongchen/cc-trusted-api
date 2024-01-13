use anyhow::*;
use core::result::Result;
use core::result::Result::Ok;

use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::eventlog::TcgEventLog;
use cctrusted_base::tcg::{TcgDigest, ALGO_NAME_MAP};

use crate::cvm::build_cvm;
use cctrusted_base::api::*;
use cctrusted_base::api_data::*;

pub struct API {}

impl CCTrustedApi for API {
    // CCTrustedApi trait function: get report of a CVM
    fn get_cc_report(
        nonce: Option<String>,
        data: Option<String>,
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
            Err(e) => Err(anyhow!("[get_cc_report] error create cvm: {:?}", e)),
        }
    }

    // CCTrustedApi trait function: dump report of a CVM in hex and char format
    fn dump_cc_report(report: &Vec<u8>) {
        dump_data(report)
    }

    // CCTrustedApi trait function: get max number of CVM IMRs
    fn get_measurement_count() -> Result<u8, anyhow::Error> {
        match build_cvm() {
            Ok(cvm) => Ok(cvm.get_max_index() + 1),
            Err(e) => Err(anyhow!("[get_measurement_count] error create cvm: {:?}", e)),
        }
    }

    // CCTrustedApi trait function: get measurements of a CVM
    fn get_cc_measurement(index: u8, algo_id: u8) -> Result<TcgDigest, anyhow::Error> {
        match build_cvm() {
            Ok(cvm) => cvm.process_cc_measurement(index, algo_id),
            Err(e) => Err(anyhow!("[get_cc_measurement] error create cvm: {:?}", e)),
        }
    }

    // CCTrustedApi trait function: get eventlogs of a CVM
    fn get_cc_eventlog(_start: u16, _count: u16) -> TcgEventLog {
        todo!()
    }

    // CCTrustedApi trait function: get default algorithm of a CVM
    fn get_default_algorithm() -> Result<Algorithm, anyhow::Error> {
        match build_cvm() {
            Ok(cvm) => {
                // call CVM trait defined methods
                let algo_id = cvm.get_algorithm_id();
                Ok(Algorithm {
                    algo_id,
                    algo_id_str: ALGO_NAME_MAP.get(&algo_id).unwrap().to_owned(),
                })
            }
            Err(e) => Err(anyhow!(
                "[get_default_algorithm] error get algorithm: {:?}",
                e
            )),
        }
    }
}

#[cfg(test)]
mod sdk_api_tests {
    use super::*;
    use crate::cvm::get_cvm_type;
    use cctrusted_base::cc_type::TeeType;
    use cctrusted_base::tcg::{TPM_ALG_SHA256, TPM_ALG_SHA384};
    use cctrusted_base::tdx::common::{Tdx,IntelTeeType,QE_VENDOR_INTEL_SGX,AttestationKeyType};
    use cctrusted_base::tdx::quote::TdxQuote;
    use log::*;
    use rand::Rng;

    // test on cc trusted API [get_cc_report]
    #[test]
    fn test_get_cc_report() {
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        let data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        match Tdx::generate_tdx_report_data(Some(nonce.clone()), Some(data.clone())) {
            Ok(r) => r,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        let report = match API::get_cc_report(Some(nonce.clone()), Some(data.clone()), ExtraArgs {})
        {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        assert_ne!(report.cc_report.len(), 0);

        let expected_cvm_type = get_cvm_type().tee_type;
        assert_eq!(report.cc_type, expected_cvm_type);
    }

    #[test]
    fn test_get_cc_report_without_data() {
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let expected_report_data = match Tdx::generate_tdx_report_data(Some(nonce.clone()), None) {
            Ok(r) => r,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        let report = match API::get_cc_report(Some(nonce.clone()), None, ExtraArgs {}) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if report.cc_type == TeeType::TDX {
            let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
                Ok(q) => q,
                Err(e) => {
                    assert_eq!(true, format!("{:?}", e).is_empty());
                    return;
                }
            };

            assert_eq!(
                base64::encode(&tdx_quote.body.report_data),
                expected_report_data
            );
        }
    }

    #[test]
    fn test_get_cc_report_without_nonce_and_data() {
        let expected_report_data = match Tdx::generate_tdx_report_data(None, None) {
            Ok(r) => r,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        let report = match API::get_cc_report(None, None, ExtraArgs {}) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if report.cc_type == TeeType::TDX {
            let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
                Ok(q) => q,
                Err(e) => {
                    assert_eq!(true, format!("{:?}", e).is_empty());
                    return;
                }
            };

            assert_eq!(
                base64::encode(&tdx_quote.body.report_data),
                expected_report_data
            );
        }
    }

    #[test]
    fn test_get_cc_report_nonce_not_base64_encoded() {
        let nonce = "XD^%*!x".to_string();
        match API::get_cc_report(Some(nonce), None, ExtraArgs {}) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(
                    true,
                    format!("{:?}", e).contains("nonce is not base64 encoded")
                );
                return;
            }
        };
    }

    #[test]
    fn test_get_cc_report_data_not_base64_encoded() {
        let data = "XD^%*!x".to_string();
        match API::get_cc_report(None, Some(data), ExtraArgs {}) {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(
                    true,
                    format!("{:?}", e).contains("data is not base64 encoded")
                );
                return;
            }
        };
    }

    // test on cc trusted API [get_default_algorithm]
    #[test]
    fn test_get_default_algorithm() {
        let defalt_algo = match API::get_default_algorithm() {
            Ok(algorithm) => algorithm,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if get_cvm_type().tee_type == TeeType::TDX {
            assert_eq!(defalt_algo.algo_id, TPM_ALG_SHA384);
        }
    }

    // test on cc trusted API [get_measurement_count]
    #[test]
    fn test_get_measurement_count() {
        let count = match API::get_measurement_count() {
            Ok(count) => count,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if get_cvm_type().tee_type == TeeType::TDX {
            assert_eq!(count, 4);
        }
    }

    // test on cc trusted API [get_cc_measurement]
    #[test]
    fn test_get_cc_measurement() {
        let count = match API::get_measurement_count() {
            Ok(count) => count,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if get_cvm_type().tee_type == TeeType::TDX {
            for index in 0..count {
                let tcg_digest = match API::get_cc_measurement(index, TPM_ALG_SHA384) {
                    Ok(tcg_digest) => tcg_digest,
                    Err(e) => {
                        assert_eq!(true, format!("{:?}", e).is_empty());
                        return;
                    }
                };

                assert_eq!(tcg_digest.algo_id, TPM_ALG_SHA384);
                assert_eq!(tcg_digest.hash.len(), 48);
            }
        }
    }

    #[test]
    fn test_get_cc_measurement_with_wrong_algo_id() {
        let count = match API::get_measurement_count() {
            Ok(count) => count,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if get_cvm_type().tee_type == TeeType::TDX {
            for index in 0..count {
                match API::get_cc_measurement(index, TPM_ALG_SHA256) {
                    Ok(tcg_digest) => tcg_digest,
                    Err(e) => {
                        assert_eq!(true, format!("{:?}", e).contains("invalid algo id"));
                        return;
                    }
                };
            }
        }
    }

    // test on cc trusted API [parse_cc_report]
    #[test]
    fn test_parse_cc_report() {
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        let data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let expected_report_data =
            match Tdx::generate_tdx_report_data(Some(nonce.clone()), Some(data.clone())) {
                Ok(r) => r,
                Err(e) => {
                    assert_eq!(true, format!("{:?}", e).is_empty());
                    return;
                }
            };

        let report = match API::get_cc_report(Some(nonce.clone()), Some(data.clone()), ExtraArgs {})
        {
            Ok(q) => q,
            Err(e) => {
                assert_eq!(true, format!("{:?}", e).is_empty());
                return;
            }
        };

        if report.cc_type == TeeType::TDX {
            let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
                Ok(q) => q,
                Err(e) => {
                    assert_eq!(true, format!("{:?}", e).is_empty());
                    return;
                }
            };

            assert_eq!(tdx_quote.header.version, 4);
            assert_eq!(tdx_quote.header.tee_type, IntelTeeType::TEE_TDX);
            assert_eq!(tdx_quote.header.qe_vendor, QE_VENDOR_INTEL_SGX);
            assert_eq!(
                base64::encode(&tdx_quote.body.report_data),
                expected_report_data
            );

            if tdx_quote.header.ak_type == AttestationKeyType::ECDSA_P256 {
                match tdx_quote.tdx_quote_ecdsa256_sigature =
                    Some(tdx_quote_ecdsa256_sigature) => {
                        assert!("tdx_quote_ecdsa256_sigature is Some");
                    }
                    None => {
                        assert!("tdx_quote_ecdsa256_sigature is None");
                    }
            }
               
        }
    }
}
