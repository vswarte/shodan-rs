use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use std::collections::HashMap;

trait Scanning {
    fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, reqwest::Error>;
    fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, reqwest::Error>;
}

impl Scanning for ShodanClient {
    fn get_scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/ports", None))
    }

    fn get_scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/protocols", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::response::ShodanClientResponse;
    use crate::scanning::Scanning;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_ports() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_scanning_ports().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }), "Response was {:?}", response);
    }

    #[test]
    fn can_get_protocols() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_scanning_protocols().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }), "Response was {:?}", response);
    }
}
