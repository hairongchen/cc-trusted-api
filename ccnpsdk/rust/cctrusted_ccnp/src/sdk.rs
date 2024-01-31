use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use cctrusted_base::api_data::Algorithm;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::tcg::EventLogEntry;
use core::result::Result::Ok;
use base64;
use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::api_data::ReplayResult;
use crate::client::CcnpServiceClient;

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

        let ccnp_service_client = CcnpServiceClient{
            uds_path: UDS_PATH.to_string(),
            client_connection: CcnpClient::new(),
        };

        let response = match ccnp_service_client.get_cc_report_from_server(nonce, data, extra_args){
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] err get cc report: {:?}", e));
            }
        };

        //FIXME: ccnp server return quote format should be enhanced
        let cc_report = match base64::decode(&response.quote.trim_matches('\"')) {
                Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] cc report is not base64 encoded: {:?}", e));
            }
        };

        //FIXME: ccnp server return TeeType directly
        let cc_type = ccnp_service_client.get_tee_type_by_name(&response.quote_type);

        Ok(CcReport{
            cc_report,
            cc_type
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