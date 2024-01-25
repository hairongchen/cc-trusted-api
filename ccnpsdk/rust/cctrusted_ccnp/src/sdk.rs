use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use cctrusted_base::api_data::Algorithm;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::tcg::EventLogEntry;
use core::result::Result::Ok;
use cctrusted_base::cc_type::TeeType;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use base64;
use tower::service_fn;
use tokio::net::UnixStream;
use crate::client::CcnpClient;

pub struct API {}

impl CCTrustedApi for API {
//impl API {
    // CCTrustedApi trait function: get cc report from CCNP server
    fn get_cc_report(
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {

        let ccnp_client = CcnpClient{
            uds_path: "/run/ccnp/uds/quote-server.sock".to_string(),
        };

        let response = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(ccnp_client.get_cc_report_from_server(
            nonce,
            data,
            extra_args
        ));

        let cc_report = match base64::decode(std::str::from_utf8(&response.quote).unwrap().trim_matches('\"')) {
            Ok(q) => q,
            Err(e) => {
                info!("cc report is not base64 encoded: {:?}", e);
                return;
            }
        };

        Ok(CcReport{
            //cc_report: cc_report,
            cc_report: response.unwrap().quote.into(),
            cc_type: TeeType::TDX
        })
    }

    // CCTrustedApi trait function: get max number of CVM IMRs
    fn get_measurement_count() -> Result<u8, anyhow::Error> {
        todo!()
    }

    // CCTrustedApi trait function: dump report of a CVM in hex and char format
    fn dump_cc_report(report: &Vec<u8>) {
        todo!()
    }

    // CCTrustedApi trait function: get measurements of a CVM
    fn get_cc_measurement(index: u8, algo_id: u16) -> Result<TcgDigest, anyhow::Error> {
        todo!()
    }

    // CCTrustedApi trait function: get eventlogs of a CVM
    fn get_cc_eventlog(
        start: Option<u32>,
        count: Option<u32>,
    ) -> Result<Vec<EventLogEntry>, anyhow::Error> {
        todo!()
    }

    // CCTrustedApi trait function: get default algorithm of a CVM
    fn get_default_algorithm() -> Result<Algorithm, anyhow::Error> {
        todo!()
    }
}