use crate::error::ShodanError;
use crate::ShodanClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Utility {
    async fn get_my_ip(&self) -> Result<String, ShodanError>;
    async fn get_http_headers(&self) -> Result<HashMap<String, String>, ShodanError>;
}

#[async_trait]
impl Utility for ShodanClient {
    async fn get_my_ip(&self) -> Result<String, ShodanError> {
        Self::fetch(self.build_request_url("/tools/myip", None)).await
    }

    async fn get_http_headers(&self) -> Result<HashMap<String, String>, ShodanError> {
        Self::fetch(self.build_request_url("/tools/httpheaders", None)).await
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::get_test_api_key;
    use crate::utility::Utility;
    use crate::ShodanClient;

    #[tokio::test]
    async fn can_get_my_ip() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_my_ip().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_http_headers() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_http_headers().await.unwrap();
    }
}
