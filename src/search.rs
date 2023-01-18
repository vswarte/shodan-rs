use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use serde::Deserialize;

trait Search {
    fn get_search_host_ip(
        &self,
        ip: String,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error>;
}

#[derive(Deserialize, Debug)]
pub struct SearchHostIpResponse {
    pub last_update: String,

    pub ip: u32,
    pub ip_str: String,
    pub ports: Vec<u16>,
    pub isp: Option<String>,
    pub asn: Option<String>,
    pub os: Option<String>,
    pub domains: Vec<String>,
    pub hostnames: Vec<String>,

    pub org: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub country_code: Option<String>,
    pub country_code_3: Option<String>,
    pub country_name: Option<String>,
    pub region_code: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub dma_code: Option<u32>,
    pub tags: Vec<String>,
    pub area_code: Option<String>,
}

impl Search for ShodanClient {
    fn get_search_host_ip(
        &self,
        ip: String,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error> {
        Self::fetch(self.build_request_url(format!("/shodan/host/{}", ip).as_str(), None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::response::ShodanClientResponse;
    use crate::search::Search;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_google_host_ip() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_search_host_ip(String::from("8.8.8.8")).unwrap();
        println!("{:?}", response);

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }
}
