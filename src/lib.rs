use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;
use url::Url;

mod parameter;
use parameter::*;

mod response;
pub use response::*;

mod builders;
pub use builders::*;

const BASE_API_URL: &str = "https://api.shodan.io";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Shodan API error: {0}")]
    Shodan(String),

    #[error("Caught reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct ShodanClient {
    api_key: String,
}

impl ShodanClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn account_profile(&self) -> Result<AccountProfileResponse> {
        Self::fetch(self.build_request_url("/account/profile", &ParameterBag::empty())?).await
    }

    pub async fn api_info(&self) -> Result<ApiInfoResponse> {
        Self::fetch(self.build_request_url("/api-info", &ParameterBag::empty())?).await
    }

    pub async fn directory_query(
        &self,
        page: Option<u32>,
        sort: Option<&str>,
        order: Option<&str>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("page", page);
        parameters.set_optional("sort", sort);
        parameters.set_optional("order", order);

        Self::fetch(self.build_request_url("/shodan/query", &parameters)?).await
    }

    pub async fn directory_query_search(
        &self,
        query: &str,
        page: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>> {
        let mut parameters = ParameterBag::default();
        parameters.set("query", query);
        parameters.set_optional("page", page);

        Self::fetch(self.build_request_url("/shodan/query/search", &parameters)?).await
    }

    pub async fn directory_query_tags(
        &self,
        size: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("size", size);

        Self::fetch(self.build_request_url("/shodan/query/tags", &parameters)?).await
    }

    pub async fn dns_domain(
        &self,
        domain: &str,
        history: Option<bool>,
        dns_type: Option<&str>,
        page: Option<u32>,
    ) -> Result<DnsDomainResponse> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("history", history);
        parameters.set_optional("dns_type", dns_type);
        parameters.set_optional("page", page);

        Self::fetch(self.build_request_url(format!("/dns/domain/{domain}").as_str(), &parameters)?)
            .await
    }

    pub async fn dns_resolve(&self, hostnames: &[&str]) -> Result<HashMap<String, Option<String>>> {
        let mut parameters = ParameterBag::default();
        parameters.set("hostnames", hostnames.join(","));

        Self::fetch(self.build_request_url("/dns/resolve", &parameters)?).await
    }

    pub async fn dns_reverse(&self, ips: &[&str]) -> Result<HashMap<String, Vec<String>>> {
        let mut parameters = ParameterBag::default();
        parameters.set("ips", ips.join(","));

        Self::fetch(self.build_request_url("/dns/reverse", &parameters)?).await
    }

    pub async fn scanning_ports(&self) -> Result<ShodanClientResponse<Vec<u16>>> {
        Self::fetch(self.build_request_url("/shodan/ports", &ParameterBag::empty())?).await
    }

    pub async fn scanning_protocols(
        &self,
    ) -> Result<ShodanClientResponse<HashMap<String, String>>> {
        Self::fetch(self.build_request_url("/shodan/protocols", &ParameterBag::empty())?).await
    }

    pub async fn host_ip(
        &self,
        ip: &str,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<SearchHostIpResponse> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("history", history);
        parameters.set_optional("minifi", minifi);

        Self::fetch(self.build_request_url(format!("/shodan/host/{ip}").as_str(), &parameters)?)
            .await
    }

    pub async fn host_search(
        &self,
        query: &str,
        facets: Option<&str>,
        page: Option<u32>,
        minifi: Option<bool>,
    ) -> Result<SearchResult> {
        let mut parameters = ParameterBag::default();
        parameters.set("query", query);
        parameters.set_optional("facets", facets);
        parameters.set_optional("page", page);
        parameters.set_optional("minifi", minifi);

        Self::fetch(self.build_request_url("/shodan/host/search", &parameters)?).await
    }

    pub async fn host_count(&self, query: &str, facets: Option<&str>) -> Result<CountResponse> {
        let mut parameters = ParameterBag::default();
        parameters.set("query", query);
        parameters.set_optional("facets", facets);

        Self::fetch(self.build_request_url("/shodan/host/count", &parameters)?).await
    }

    pub async fn host_facets(&self) -> Result<Vec<String>> {
        Self::fetch(self.build_request_url("/shodan/host/search/facets", &ParameterBag::empty())?)
            .await
    }

    pub async fn host_filters(&self) -> Result<Vec<String>> {
        Self::fetch(self.build_request_url("/shodan/host/search/filters", &ParameterBag::empty())?)
            .await
    }

    pub async fn host_tokens(&self, query: &str) -> Result<TokenResponse> {
        let mut parameters = ParameterBag::default();
        parameters.set("query", query);

        Self::fetch(self.build_request_url("/shodan/host/search/tokens", &parameters)?).await
    }

    pub async fn my_ip(&self) -> Result<String> {
        Self::fetch(self.build_request_url("/tools/myip", &ParameterBag::empty())?).await
    }

    pub async fn http_headers(&self) -> Result<HashMap<String, String>> {
        Self::fetch(self.build_request_url("/tools/httpheaders", &ParameterBag::empty())?).await
    }

    fn build_request_url(&self, endpoint: &str, parameters: &ParameterBag) -> Result<String> {
        let mut url = Url::parse(BASE_API_URL)?;
        url.set_path(endpoint);

        // Set API key
        url.query_pairs_mut()
            .append_pair("key", self.api_key.as_str());

        // Set any additional parameters
        url.query_pairs_mut().extend_pairs(parameters.pairs());

        Ok(url.to_string())
    }

    async fn fetch<T: for<'a> Deserialize<'a>>(url: String) -> Result<T> {
        let response = reqwest::get(url)
            .await?
            .json::<ShodanClientResponse<T>>()
            .await?;

        match response {
            ShodanClientResponse::Error(e) => {
                Err(Error::Shodan(format!("Error response: {}", e.error)))
            }
            ShodanClientResponse::Response(r) => Ok(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    pub fn get_test_api_key() -> String {
        let api_key = env::var("SHODAN_TEST_KEY");
        match api_key {
            Ok(key) => key,
            Err(_) => panic!("Did not specify a shodan API key for testing"),
        }
    }

    #[tokio::test]
    async fn can_get_account_profile() {
        let client = ShodanClient::new(get_test_api_key());
        client.account_profile().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_api_info() {
        let client = ShodanClient::new(get_test_api_key());
        client.api_info().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_directory_query() {
        let client = ShodanClient::new(get_test_api_key());
        client.directory_query(None, None, None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_directory_query_search() {
        let client = ShodanClient::new(get_test_api_key());
        client.directory_query_search("webcam", None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_directory_query_tags() {
        let client = ShodanClient::new(get_test_api_key());
        client.directory_query_tags(None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_dns_domain() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .dns_domain("google.com", None, None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_dns_resolve() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .dns_resolve(&["google.com", "facebook.com"])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_dns_reverse() {
        let client = ShodanClient::new(get_test_api_key());
        client.dns_reverse(&["8.8.8.8", "1.1.1.1"]).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_ports() {
        let client = ShodanClient::new(get_test_api_key());
        client.scanning_ports().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_protocols() {
        let client = ShodanClient::new(get_test_api_key());
        client.scanning_protocols().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_google_host_ip() {
        let client = ShodanClient::new(get_test_api_key());
        client.host_ip("8.8.8.8", None, None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_host_facets() {
        let client = ShodanClient::new(get_test_api_key());
        client.host_facets().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_host_filters() {
        let client = ShodanClient::new(get_test_api_key());
        client.host_filters().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_google_count() {
        let client = ShodanClient::new(get_test_api_key());
        client.host_count("google", None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_google_count_with_facets() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .host_count("google", Some("os,country"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_google_search() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .host_search("google", None, None, Some(true))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_raspbian_tokens() {
        let client = ShodanClient::new(get_test_api_key());
        client.host_tokens("Raspbian port:22").await.unwrap();
    }

    #[tokio::test]
    async fn can_get_my_ip() {
        let client = ShodanClient::new(get_test_api_key());
        client.my_ip().await.unwrap();
    }

    #[tokio::test]
    async fn can_get_http_headers() {
        let client = ShodanClient::new(get_test_api_key());
        client.http_headers().await.unwrap();
    }
}
