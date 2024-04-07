use crate::error::Error;
use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Scanning {
    async fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, Error>;

    async fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, Error>;
}

#[async_trait]
impl Scanning for ShodanClient {
    async fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, Error> {
        Self::fetch(self.build_request_url("/shodan/ports", &Default::default())?).await
    }

    async fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, Error> {
        Self::fetch(self.build_request_url("/shodan/protocols", &Default::default())?).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

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
