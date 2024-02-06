use cctrusted_base::api_data::ExtraArgs;
use core::result::Result::Ok;
use tonic::transport::{Endpoint, Uri};
use tonic::Request;
use tower::service_fn;
use crate::client::ccnp_server_pb::ccnp_client::CcnpClient;
use crate::client::ccnp_server_pb::GetCcReportRequest;
use crate::client::ccnp_server_pb::GetCcReportResponse;
use tokio::net::UnixStream;
use cctrusted_base::cc_type::TeeType;
use hashbrown::HashMap;

lazy_static! {
    pub static ref TEE_NAME_TYPE_MAP: HashMap<u32, TeeType> = {
        let mut map: HashMap<u32, TeeType> = HashMap::new();
        map.insert(1, TeeType::TPM);
        map.insert(2, TeeType::TDX, );
        map.insert(3, TeeType::SEV);
        map.insert(4, TeeType::CCA);
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
    ) -> Result<GetCcReportResponse, anyhow::Error> {

        let uds_path = self.ccnp_uds_path.parse::<Uri>().unwrap();
        let channel = Endpoint::try_from("http://[::]:0")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(uds_path.to_string())
            }))
            .await
            .unwrap();

        let request = Request::new(GetCcReportRequest {
            nonce: nonce.unwrap(),
            user_data: data.unwrap()
        });

        let mut ccnp_client = CcnpClient::new(channel);

        let response = ccnp_client.get_cc_report(request).await.unwrap().into_inner();
        Ok(response)
    }

    // turn async call to sync call
    pub fn get_cc_report_from_server(
        &mut self,
        nonce: Option<String>,
        data: Option<String>,
        extra_args: ExtraArgs,
    ) -> Result<GetCcReportResponse, anyhow::Error> {
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

    pub fn get_tee_type_by_value(&self, tee_id: u8) -> TeeType {
        match TEE_NAME_TYPE_MAP.get(tee_id) {
            Some(tee_type) => tee_type.clone(),
            None => TeeType::PLAIN,
        }
    }
}