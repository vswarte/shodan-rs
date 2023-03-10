use async_trait::async_trait;
use crate::error::ShodanError;
use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use std::collections::HashMap;

#[async_trait]
pub trait Scanning {
    async fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, ShodanError>;

    async fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, ShodanError>;
}

#[async_trait]
impl Scanning for ShodanClient {
    async fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, ShodanError> {
        Self::fetch(self.build_request_url("/shodan/ports", None)).await
    }

    async fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, ShodanError> {
        Self::fetch(self.build_request_url("/shodan/protocols", None)).await
    }
}

#[cfg(test)]
pub mod tests {
    use crate::scanning::Scanning;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[tokio::test]
    async fn can_get_ports() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_scanning_ports().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_protocols() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_scanning_protocols().await.unwrap();
    }
}
