use crate::error::ShodanError;
use crate::ShodanClient;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponse {
    pub scan_credits: u32,
    pub usage_limits: ApiInfoResponseUsageLimits,
    pub plan: String,
    pub https: bool,
    pub unlocked: bool,
    pub query_credits: u32,
    pub monitored_ips: Option<u32>,
    pub unlocked_left: u32,
    pub telnet: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApiInfoResponseUsageLimits {
    pub scan_credits: i32,
    pub query_credits: i32,
    pub monitored_ips: i32,
}

#[async_trait]
pub trait ApiInfo {
    async fn get_api_info(&self) -> Result<ApiInfoResponse, ShodanError>;
}

#[async_trait]
impl ApiInfo for ShodanClient {
    async fn get_api_info(&self) -> Result<ApiInfoResponse, ShodanError> {
        Self::fetch(self.build_request_url("/api-info", None)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

    #[tokio::test]
    async fn can_get_api_info() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_api_info().await.unwrap();
    }
}
