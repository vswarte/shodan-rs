use std::collections::HashMap;
use crate::response::ShodanClientResponse;
use crate::{add_parameter, ShodanClient};
use serde::Deserialize;

trait Search {
    fn search_host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error>;

    fn get_search_host_facets(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error>;

    fn get_search_host_filters(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error>;
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
    fn search_host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error> {
        let mut parameters = HashMap::new();
        add_parameter("history", history, &mut parameters);
        add_parameter("minifi", minifi, &mut parameters);
        Self::fetch(self.build_request_url(format!("/shodan/host/{ip}").as_str(), Some(parameters)))
    }

    fn get_search_host_facets(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/host/search/facets", None))
    }

    fn get_search_host_filters(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/host/search/filters", None))
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
        let response = client.search_host_ip(String::from("8.8.8.8"), None, None).unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_host_facets() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_search_host_facets().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_host_filters() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_search_host_filters().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }
}
