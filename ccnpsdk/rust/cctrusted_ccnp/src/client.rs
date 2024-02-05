use cctrusted_base::api_data::ExtraArgs;
use core::result::Result::Ok;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use tower::service_fn;
use crate::client::ccnp_server_pb::ccnp_client::CcnpClient;
use crate::client::ccnp_server_pb::GetQuoteRequest;
use crate::client::ccnp_server_pb::GetQuoteResponse;
use tokio::net::UnixStream;
use cctrusted_base::cc_type::TeeType;
use hashbrown::HashMap;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

static CLIENT: OnceCell<CcnpClient<Channel>> = OnceCell::const_new();
async fn get_client(ccnp_uds_path: String) -> CcnpClient<Channel> {
    let uds_path = ccnp_uds_path.parse::<Uri>().unwrap();
    CLIENT.get_or_init(|| async {
        info!(get_or_init);
        let channel = Endpoint::try_from("http://[::]:0")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            UnixStream::connect(uds_path.to_string())
        }))
        .await
        .unwrap();

        CcnpClient::new(channel)
    })
    .await.clone()
}


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

pub struct CcnpServiceClient{
    pub ccnp_uds_path: String,
}

impl CcnpServiceClient {
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

        //let mut ccnp_client = CcnpClient::new(channel);
        let mut ccnp_client = get_client(self.ccnp_uds_path.clone()).await;

        let response = ccnp_client.get_quote(request).await.unwrap().into_inner();
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