use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use cctrusted_base::api_data::Algorithm;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::tcg::EventLogEntry;
use core::result::Result::Ok;
use cctrusted_base::cc_type::TeeType;
use base64;
use crate::client::CcnpClient;
use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::api_data::ReplayResult;

const UDS_PATH: &str = "/run/ccnp/uds/quote-server.sock";

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
            uds_path: UDS_PATH.to_string(),
        };

        let response = match ccnp_client.get_cc_report_from_server(nonce, data, extra_args){
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] err get cc report: {:?}", e));
            }
        };

        //TODO: need to fix the quote server response
        let report = match base64::decode(&response.quote.trim_matches('\"')) {
                Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] cc report is not base64 encoded: {:?}", e));
            }
        };

        Ok(CcReport{
            cc_report: report,
            //TODO: need to fix 
            cc_type: TeeType::TDX
        })
    }

    // CCTrustedApi trait function: dump report of a CVM in hex and char format
    fn dump_cc_report(report: &Vec<u8>) {
        dump_data(report)
    }

    // CCTrustedApi trait function: get max number of CVM IMRs
    fn get_measurement_count() -> Result<u8, anyhow::Error> {
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

    // CCTrustedApi trait function: replay eventlogs of a CVM
    fn replay_cc_eventlog(
        eventlogs: Vec<EventLogEntry>,
    ) -> Result<Vec<ReplayResult>, anyhow::Error> {
        todo!()
    }

    // CCTrustedApi trait function: get default algorithm of a CVM
    fn get_default_algorithm() -> Result<Algorithm, anyhow::Error> {
        todo!()
    }
}