pub struct CcnpClient{
    pub uds_path: String;
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