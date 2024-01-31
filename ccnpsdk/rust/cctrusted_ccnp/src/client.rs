use cctrusted_base::api_data::ExtraArgs;
use core::result::Result::Ok;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use tower::service_fn;
use crate::client::quote_server::get_quote_client::GetQuoteClient;
use crate::client::quote_server::GetQuoteRequest;
use crate::client::quote_server::GetQuoteResponse;
use tokio::net::UnixStream;
use cctrusted_base::cc_type::TeeType;
use hashbrown::HashMap;

lazy_static! {
    pub static ref TEE_NAME_TYPE_MAP: HashMap<String, TeeType> = {
        let mut map: HashMap<String, TeeType> = HashMap::new();
        map.insert("PLAIN".to_string(), TeeType::PLAIN);
        map.insert("TDX".to_string(), TeeType::TDX, );
        map.insert("SEV".to_string(), TeeType::SEV);
        map.insert("CCA".to_string(), TeeType::CCA);
        map.insert("TPM".to_string(), TeeType::TPM);
        map
    };
}

pub mod quote_server {
    tonic::include_proto!("quoteserver");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("quote_server_descriptor");
}

pub struct CcnpClient{
    pub uds_path: String,
}

impl CcnpClient {
    async fn get_cc_report_from_server_async(
        &self,
        nonce: Option<String>,
        data: Option<String>,
        _extra_args: ExtraArgs,
    ) -> Result<GetQuoteResponse, anyhow::Error> {
        let uds_path = (&self.uds_path).parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
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

    // turn async call to sync call
    pub fn get_cc_report_from_server(
        &self,
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<GetQuoteResponse, anyhow::Error> {
        let response = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(self.get_cc_report_from_server_async(
            nonce,
            data,
            extra_args
        ));
        response
    }

    pub fn get_tee_type_by_name(&self, tee_name: &String) -> TeeType {
        match TEE_NAME_TYPE_MAP.get(tee_name) {
            Some(tee_type) => tee_type.clone(),
            None => TeeType::PLAIN,
        }
    }
}