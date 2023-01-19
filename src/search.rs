use crate::response::ShodanClientResponse;
use crate::{add_optional_parameter, ShodanClient};
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

trait Search {
    fn search_host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error>;

    fn search_host_search(
        &self,
        query: String,
        facets: Option<String>,
        page: Option<i32>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchResponse>, reqwest::Error>;

    fn search_host_count(
        &self,
        query: String,
        facets: Option<String>,
    ) -> Result<ShodanClientResponse<CountResponse>, reqwest::Error>;

    fn search_host_facets(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error>;

    fn search_host_filters(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error>;
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

#[derive(Deserialize, Debug)]
pub struct Match {
    pub asn: Option<String>,
    pub os: Option<String>,
    pub domains: Vec<String>,
    pub hostnames: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub matches: Vec<Match>,
    pub total: u32,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Deserialize, Debug)]
pub struct CountResponse {
    pub total: u32,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Deserialize, Debug)]
pub struct Facet {
    pub count: u32,
    pub value: String,
}

impl Search for ShodanClient {
    fn search_host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchHostIpResponse>, reqwest::Error> {
        let mut parameters = HashMap::new();
        add_optional_parameter("history", history, &mut parameters);
        add_optional_parameter("minifi", minifi, &mut parameters);

        Self::fetch(self.build_request_url(format!("/shodan/host/{ip}").as_str(), Some(parameters)))
    }

    fn search_host_search(
        &self,
        query: String,
        facets: Option<String>,
        page: Option<i32>,
        minifi: Option<bool>,
    ) -> Result<ShodanClientResponse<SearchResponse>, Error> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("facets", facets, &mut parameters);
        add_optional_parameter("page", page, &mut parameters);
        add_optional_parameter("minifi", minifi, &mut parameters);

        Self::fetch(
            self.build_request_url(format!("/shodan/host/search").as_str(), Some(parameters)),
        )
    }

    fn search_host_count(
        &self,
        query: String,
        facets: Option<String>,
    ) -> Result<ShodanClientResponse<CountResponse>, Error> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("facets", facets, &mut parameters);

        Self::fetch(
            self.build_request_url(format!("/shodan/host/count").as_str(), Some(parameters)),
        )
    }

    fn search_host_facets(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/host/search/facets", None))
    }

    fn search_host_filters(&self) -> Result<ShodanClientResponse<Vec<String>>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/host/search/filters", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::response::ShodanClientResponse;
    use crate::search::*;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_google_host_ip() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .search_host_ip(String::from("8.8.8.8"), None, None)
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_host_facets() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.search_host_facets().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_host_filters() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.search_host_filters().unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_google_count() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .search_host_count(String::from("google"), None)
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_google_count_with_facets() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .search_host_count(String::from("google"), Some(String::from("country")))
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_google_search() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .search_host_search(String::from("google"), None, None, None)
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }
}
