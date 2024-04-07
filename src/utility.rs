use crate::*;
use async_trait::async_trait;

#[async_trait]
pub trait Utility {
    async fn get_my_ip(&self) -> Result<String, Error>;
    async fn get_http_headers(&self) -> Result<HashMap<String, String>, Error>;
}

#[async_trait]
impl Utility for ShodanClient {
    async fn get_my_ip(&self) -> Result<String, Error> {
        Self::fetch(self.build_request_url("/tools/myip", &Default::default())?).await
    }

    async fn get_http_headers(&self) -> Result<HashMap<String, String>, Error> {
        Self::fetch(self.build_request_url("/tools/httpheaders", &Default::default())?).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

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
