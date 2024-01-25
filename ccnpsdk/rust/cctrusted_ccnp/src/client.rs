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
use crate::sdk::quote_server::get_quote_client::GetQuoteClient;
use crate::sdk::quote_server::GetQuoteRequest;
use tokio::net::UnixStream;

pub mod quote_server {
    tonic::include_proto!("quoteserver");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("quote_server_descriptor");
}

pub struct CcnpClient{
    pub uds_path: String,
}

impl CcnpClient {
    pub async fn get_cc_report_from_server(
        &self,
        nonce: Option<String>,
        data: Option<String>,
        _extra_args: ExtraArgs,
    ) -> Result<CcReport, anyhow::Error> {
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(|_: Uri| {
                UnixStream::connect(&self.uds_path)
            }))
            .await
            .unwrap();
    
        let mut client = GetQuoteClient::new(channel);
    
        let request = Request::new(GetQuoteRequest {
            nonce: nonce.unwrap(),
            user_data: data.unwrap()
        });
    
        let response = client.get_quote(request).await.unwrap().into_inner();
        Ok(response)
    }
}