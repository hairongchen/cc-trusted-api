use anyhow::*;
//use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use cctrusted_base::api_data::Algorithm;
use cctrusted_base::tcg::TcgDigest;
use cctrusted_base::tcg::EventLogEntry;
use std::os::unix::net::UnixStream;
use core::result::Result::Ok;
use cctrusted_base::cc_type::TeeType;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use base64;
use tower::service_fn;
use sdk::get_quote_server::{GetQuote, GetQuoteServer};
use sdk::{GetQuoteRequest, GetQuoteResponse};

pub mod quote_server {
    tonic::include_proto!("quoteserver");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("quote_server_descriptor");
}

pub struct API {}

//impl CCTrustedApi for API {
impl API {
        // CCTrustedApi trait function: get cc report from CCNP server
    async fn get_cc_report(
        nonce: Option<String>,
        data: Option<String>,
        _extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {
        let channel = Endpoint::try_from("http://[::]:40081")
            .unwrap()
            .connect_with_connector(service_fn(|_: Uri| {
                let path = "/run/ccnp/uds/quote-server.sock";
                UnixStream::connect(path)
            }))
            .await
            .unwrap();

        let mut client = GetQuoteClient::new(channel);

        let request = Request::new(GetQuoteRequest {
            nonce,
            user_data: data
        });

        let response = client.get_quote(request).await.unwrap().into_inner();
        //let response = client.get_quote(request).unwrap().into_inner();
        let cc_report = match base64::decode(response.quote) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("cc report is not base64 encoded: {:?}", e)),
        };

        Ok(CcReport{
            cc_report: cc_report,
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