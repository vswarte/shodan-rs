use crate::error::ShodanError;
use crate::{add_optional_parameter, ShodanClient};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[async_trait]
pub trait Search {
    async fn host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<SearchHostIpResponse, ShodanError>;

    async fn host_search(
        &self,
        query: String,
        facets: Option<&str>,
        page: Option<i32>,
        minifi: Option<bool>,
    ) -> Result<SearchResult, ShodanError>;

    async fn host_count(
        &self,
        query: String,
        facets: Option<&str>,
    ) -> Result<CountResponse, ShodanError>;

    async fn host_facets(&self) -> Result<Vec<String>, ShodanError>;

    async fn host_filters(&self) -> Result<Vec<String>, ShodanError>;

    async fn host_tokens(&self, query: String) -> Result<TokenResponse, ShodanError>;
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
pub struct CountResponse {
    pub total: u32,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub attributes: HashMap<String, Vec<i32>>,
    pub errors: Vec<String>,
    pub string: String,
    pub filters: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Facet {
    pub count: u32,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub matches: Vec<SearchResultMatch>,
    pub total: i64,
    pub facets: Option<HashMap<String, Vec<Facet>>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultMatch {
    pub hash: i64,
    pub asn: Option<String>,
    pub http: Option<Http>,
    pub os: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: String,
    pub isp: Option<String>,
    pub transport: String,
    #[serde(rename = "_shodan")]
    pub shodan: Shodan,
    pub ssl: Option<Ssl>,
    pub cloud: Option<Cloud>,
    pub hostnames: Vec<String>,
    pub location: LocationClass,
    pub ip: Option<i64>,
    pub domains: Vec<String>,
    pub org: String,
    pub data: String,
    pub port: i64,
    pub ip_str: String,
    pub product: Option<String>,
    pub cpe23: Option<Vec<String>>,
    pub cpe: Option<Vec<String>>,
    pub version: Option<String>,
    //pub mysql: Option<Mysql>,
    pub info: Option<String>,
    //pub vulns: Option<HashMap<String, Vuln>>,
    pub ipv6: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Cloud {
    pub region: Option<String>,
    pub service: Option<String>,
    pub provider: String,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub status: i64,
    pub robots_hash: Option<i64>,
    pub redirects: Vec<Redirect>,
    pub securitytxt: Option<String>,
    pub title: Option<String>,
    pub sitemap_hash: Option<i64>,
    pub robots: Option<String>,
    pub server: Option<String>,
    pub headers_hash: i64,
    pub host: String,
    pub html: String,
    pub location: String,
    pub components: Option<HashMap<String, Component>>,
    pub html_hash: i64,
    pub sitemap: Option<String>,
    pub securitytxt_hash: Option<i64>,
    pub favicon: Option<Favicon>,
    pub waf: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Favicon {
    pub hash: i64,
    pub data: String,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    pub host: String,
    pub data: String,
    pub location: String,
    pub html: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationClass {
    pub city: String,
    pub region_code: String,
    pub area_code: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub authentication_plugin: String,
    pub capabilities: i64,
    pub server_status: String,
    pub thread_id: i64,
    pub version: String,
    pub extended_server_capabilities: i64,
    pub protocol_version: i64,
    pub server_language: i64,
}

#[derive(Debug, Deserialize)]
pub struct Shodan {
    pub region: String,
    pub ptr: Option<bool>,
    pub module: String,
    pub id: String,
    pub options: HashMap<String, String>,
    pub crawler: String,
}

#[derive(Debug, Deserialize)]
pub struct Ssl {
    pub chain_sha256: Vec<String>,
    pub jarm: String,
    pub chain: Vec<String>,
    pub dhparams: Option<Dhparams>,
    pub versions: Vec<String>,
    //pub acceptable_cas: Vec<String>,
    pub tlsext: Vec<Tlsext>,
    #[serde(rename = "ja3s")]
    pub ja3_s: String,
    pub cert: Cert,
    pub cipher: Cipher,
    pub trust: Trust,
    pub handshake_states: Vec<String>,
    pub alpn: Vec<String>,
    pub ocsp: Ocsp,
}

#[derive(Debug, Deserialize)]
pub struct Cert {
    pub sig_alg: String,
    pub issued: String,
    pub expires: String,
    pub expired: bool,
    pub version: i64,
    pub extensions: Vec<Extension>,
    pub fingerprint: Fingerprint,
    pub serial: f64,
    pub subject: Issuer,
    pub pubkey: Pubkey,
    pub issuer: Issuer,
}

#[derive(Debug, Deserialize)]
pub struct Extension {
    pub critical: Option<bool>,
    pub data: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Fingerprint {
    pub sha256: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct Issuer {
    #[serde(rename = "C")]
    pub c: Option<String>,
    #[serde(rename = "CN")]
    pub cn: Option<String>,
    #[serde(rename = "O")]
    pub o: Option<String>,
    #[serde(rename = "L")]
    pub l: Option<String>,
    #[serde(rename = "ST")]
    pub st: Option<String>,
    #[serde(rename = "OU")]
    pub ou: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Pubkey {
    #[serde(rename = "type")]
    pub pubkey_type: String,
    pub bits: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cipher {
    pub version: String,
    pub bits: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Dhparams {
    pub prime: String,
    pub public_key: String,
    pub bits: i64,
    pub generator: i64,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Ocsp {
    pub version: Option<String>,
    pub response_status: Option<String>,
    pub responder_id: Option<String>,
    pub cert_status: Option<String>,
    pub produced_at: Option<String>,
    pub signature_algorithm: Option<String>,
    pub next_update: Option<String>,
    pub this_update: Option<String>,
    pub certificate_id: Option<CertificateId>,
}

#[derive(Debug, Deserialize)]
pub struct CertificateId {
    pub hash_algorithm: String,
    pub issuer_name_hash: String,
    pub issuer_name_key: String,
    pub serial_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Tlsext {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Trust {
    pub revoked: bool,
    pub browser: Option<Browser>,
}

#[derive(Debug, Deserialize)]
pub struct Browser {
    pub mozilla: bool,
    pub apple: bool,
    pub microsoft: bool,
}

#[derive(Debug, Deserialize)]
pub struct Vuln {
    pub verified: bool,
    pub references: Vec<String>,
    pub cvss: Option<f64>,
    pub summary: String,
}

#[async_trait]
impl Search for ShodanClient {
    async fn host_ip(
        &self,
        ip: String,
        history: Option<bool>,
        minifi: Option<bool>,
    ) -> Result<SearchHostIpResponse, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("history", history, &mut parameters);
        add_optional_parameter("minifi", minifi, &mut parameters);

        Self::fetch(self.build_request_url(format!("/shodan/host/{ip}").as_str(), Some(parameters)))
            .await
    }

    async fn host_search(
        &self,
        query: String,
        facets: Option<&str>,
        page: Option<i32>,
        minifi: Option<bool>,
    ) -> Result<SearchResult, ShodanError> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("facets", facets, &mut parameters);
        add_optional_parameter("page", page, &mut parameters);
        add_optional_parameter("minifi", minifi, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/host/search", Some(parameters))).await
    }

    async fn host_count(
        &self,
        query: String,
        facets: Option<&str>,
    ) -> Result<CountResponse, ShodanError> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("facets", facets, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/host/count", Some(parameters))).await
    }

    async fn host_facets(&self) -> Result<Vec<String>, ShodanError> {
        Self::fetch(self.build_request_url("/shodan/host/search/facets", None)).await
    }

    async fn host_filters(&self) -> Result<Vec<String>, ShodanError> {
        Self::fetch(self.build_request_url("/shodan/host/search/filters", None)).await
    }

    async fn host_tokens(&self, query: String) -> Result<TokenResponse, ShodanError> {
        let parameters = HashMap::from([(String::from("query"), query)]);

        Self::fetch(self.build_request_url("/shodan/host/search/tokens", Some(parameters))).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

    #[tokio::test]
    async fn can_get_google_host_ip() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .host_ip(String::from("8.8.8.8"), None, None)
            .await
            .unwrap();
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
        client
            .host_count(String::from("google"), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_google_count_with_facets() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .host_count(String::from("google"), Some("os,country"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_google_search() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .host_search(String::from("google"), None, None, None)
            .await;

        match response {
            Ok(r) => {
                println!("{r:?}")
            }
            Err(e) => match e {
                ShodanError::ShodanClientError(e) => {
                    panic!("Got a shodan client error: {e}")
                }
                ShodanError::ReqwestError(e) => {
                    panic!("Got a reqwest error: {e:?}")
                }
            },
        }
    }

    #[tokio::test]
    async fn can_get_raspbian_tokens() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .host_tokens(String::from("Raspbian port:22"))
            .await
            .unwrap();
    }
}
