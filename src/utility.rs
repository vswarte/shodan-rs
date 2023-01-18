use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use std::collections::HashMap;

trait Utility {
    fn get_my_ip(&self) -> Result<ShodanClientResponse<String>, reqwest::Error>;
    fn get_http_headers(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, reqwest::Error>;
}

impl Utility for ShodanClient {
    fn get_my_ip(&self) -> Result<ShodanClientResponse<String>, reqwest::Error> {
        Self::fetch(self.build_request_url("/tools/myip", None))
    }

    fn get_http_headers(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/tools/httpheaders", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::response::ShodanClientResponse;
    use crate::tests::get_test_api_key;
    use crate::utility::Utility;
    use crate::ShodanClient;

    #[test]
    fn can_get_my_ip() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_my_ip().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_http_headers() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_http_headers().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }
}
