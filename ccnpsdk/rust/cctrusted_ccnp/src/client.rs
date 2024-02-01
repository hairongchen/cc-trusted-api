use cctrusted_base::api_data::ExtraArgs;
use core::result::Result::Ok;
use tonic::transport::{Endpoint, Uri, Channel};
use tonic::Request;
use tower::service_fn;
use crate::client::ccnp_server_pb::ccnp_client::CcnpClient;
use crate::client::ccnp_server_pb::GetQuoteRequest;
use crate::client::ccnp_server_pb::GetQuoteResponse;
use tokio::net::UnixStream;
use cctrusted_base::cc_type::TeeType;
use hashbrown::HashMap;
use log::info;

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

pub mod ccnp_server_pb {
    tonic::include_proto!("ccnp_server_pb");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ccnp_server_descriptor");
}

#[derive(Clone)]
pub struct CcnpServiceClient{
    pub ccnp_uds_path: String,
    pub client_channel: Channel
}

impl CcnpServiceClient {
    // turn async call to sync call
    pub fn new(ccnp_uds_path: String) -> Result<CcnpServiceClient, anyhow::Error> {
        let client = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(CcnpServiceClient::new_async(ccnp_uds_path));
        //Ok(client?.clone())
        // let mut client1 = client?.clone();
        let request = Request::new(GetQuoteRequest {
            nonce: "MtbxK6RXDd1vbS2++JcBZ/+Xc1DhrjRcjTd3dZ3EIZs=".to_string(),
            user_data: "4aYiL5jfw692TxSs2DrhINFhPkVLy0Edn0nCKLa9Ix8=".to_string(),
        });

        let mut cc = CcnpClient::new(client?.client_channel.clone());

        let response = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(cc.get_quote(request));

        info!("response = {}", response?.into_inner().quote_type);
        client
    }

    pub async fn new_async(ccnp_uds_path: String) -> Result<CcnpServiceClient, anyhow::Error>{
        let uds_path = ccnp_uds_path.parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
            }))
            .await
            .unwrap();
        // let request = Request::new(GetQuoteRequest {
        //     nonce: "MtbxK6RXDd1vbS2++JcBZ/+Xc1DhrjRcjTd3dZ3EIZs=".to_string(),
        //     user_data: "4aYiL5jfw692TxSs2DrhINFhPkVLy0Edn0nCKLa9Ix8=".to_string(),
        // });

        // let mut client = CcnpClient::new(channel.clone());
        // let response = client.get_quote(request).await.unwrap().into_inner();
        // info!("response = {}", response.quote_type);

        let cc = CcnpServiceClient{
                ccnp_uds_path,
                client_channel: channel.clone()
            };

        // let request = Request::new(GetQuoteRequest {
        //     nonce: "MtbxK6RXDd1vbS2++JcBZ/+Xc1DhrjRcjTd3dZ3EIZs=".to_string(),
        //     user_data: "4aYiL5jfw692TxSs2DrhINFhPkVLy0Edn0nCKLa9Ix8=".to_string(),
        // });

        // let mut client = CcnpClient::new(cc.client_channel.clone());
        // let response = client.get_quote(request).await.unwrap().into_inner();
        // info!("response = {}", response.quote_type);
        
        Ok(cc)

        // Ok(CcnpServiceClient{
        //     ccnp_uds_path,
        //     client_channel: channel.clone()
        // })
    }

    async fn get_cc_report_from_server_async(
        &mut self,
        nonce: Option<String>,
        data: Option<String>,
        _extra_args: ExtraArgs,
    ) -> Result<GetQuoteResponse, anyhow::Error> {

        let request = Request::new(GetQuoteRequest {
            nonce: nonce.unwrap(),
            user_data: data.unwrap()
        });

        let mut client = CcnpClient::new(self.client_channel.clone());

        let response = client.get_quote(request).await.unwrap().into_inner();
        Ok(response)
    }

    // turn async call to sync call
    pub fn get_cc_report_from_server(
        &mut self,
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