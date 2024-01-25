use anyhow::*;
use cctrusted_base::api::CCTrustedApi;
use cctrusted_base::api_data::ExtraArgs;
use cctrusted_base::api_data::CcReport;
use std::os::unix::net::UnixStream;
use core::error::Request;
use core::result::Result::Ok;
use cctrusted_base::cc_type::TeeType;
use tonic::transport::{Endpoint, Uri};

pub struct API {}

impl CCTrustedApi for API {
    // CCTrustedApi trait function: get cc report from CCNP server
    fn get_cc_report(
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
        let cc_report = match base64::decode(response.quote) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("cc report is not base64 encoded: {:?}", e)),
        };

        Ok(CcReport{
            cc_report: cc_report,
            cc_type: TeeType::TDX
        })

    }
}