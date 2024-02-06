use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use cctrusted_base::api_data::Algorithm;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::tcg::EventLogEntry;
use core::result::Result::Ok;
use cctrusted_base::binary_blob::dump_data;
use cctrusted_base::api_data::ReplayResult;
use crate::client::CcnpServiceClient;
use log::info;

const UDS_PATH: &str = "/run/ccnp/uds/ccnp-server.sock";

pub struct API {}

impl CCTrustedApi for API {
    // CCTrustedApi trait function: get cc report from CCNP server
    fn get_cc_report(
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {

        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string()
        };

        let response = match ccnp_service_client.get_cc_report_from_server(nonce, data, extra_args){
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_report] err get cc report: {:?}", e));
            }
        };

        Ok(CcReport{
            cc_report: response.cc_report,
            cc_type: ccnp_service_client.get_tee_type_by_value(&response.cc_type)
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
        let mut ccnp_service_client = CcnpServiceClient {
            ccnp_uds_path: UDS_PATH.to_string()
        };

        let response = match ccnp_service_client.get_cc_measurement_from_server(index, algo_id){
            Ok(r) => r,
            Err(e) => {
                return Err(anyhow!("[get_cc_measurement] err get cc measurement: {:?}", e));
            }
        };

        let measurement = match response.measurement{
            Some(measurement) => measurement,
            None => return Err(anyhow!("[get_cc_measurement] faile to get cc measurement")),
        };

        Ok(TcgDigest{
            algo_id: measurement.algo_id as u16,
            hash: measurement.hash
        })
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