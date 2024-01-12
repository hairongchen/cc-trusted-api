#[cfg(test)]
mod sdk_api_tests {
    use super::*;
    use log::*;
    use rand::Rng;
    use cctrusted_base::tdx::common::Tdx;
    use cctrusted_base::cc_type::TeeType;
    use cctrusted_base::tdx::quote::TdxQuote;


    #[test]
    // test on cc trusted API [get_cc_report]
    fn test_get_cc_report() {
        let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
        let data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

        let expected_report_data = match Tdx::generate_tdx_report_data(nonce.clone(), Some(data.clone())) {
            Ok(r) => r,
            Err(e) => {
                error!("[test_get_cc_report] error generating TDX report data: {:?}", e);
                return;
            }
        };

        let report = match API::get_cc_report(nonce, data, ExtraArgs {}) {
            Ok(q) => q,
            Err(e) => {
                error!("[test_get_cc_report] error getting TDX report: {:?}", e);
                return;
            }
        };

        if report.cc_type == TeeType::TDX {
            let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
                Ok(q) => q,
                Err(e) => {
                    error!("[test_get_cc_report] error parse tdx quote: {:?}", e);
                    return;
                }
            };

            assert_eq!(base64::encode(&tdx_quote.body.report_data), expected_report_data);
        }
    }

}