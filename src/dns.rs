use crate::error::ShodanError;
use crate::{add_optional_parameter, ShodanClient};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct DnsDomainResponse {
    pub domain: String,
    pub tags: Vec<String>,
    pub data: Vec<DnsDomainDataItem>,
    pub subdomains: Vec<String>,
    pub more: bool,
}

#[derive(Deserialize, Debug)]
pub struct DnsDomainDataItem {
    pub subdomain: String,
    #[serde(rename(deserialize = "type"))]
    pub item_type: String, // Type is a reserved keyword
    pub value: String,
    pub last_seen: String,
}

#[async_trait]
pub trait Dns {
    async fn dns_domain(
        &self,
        domain: String,
        history: Option<bool>,
        dns_type: Option<String>,
        page: Option<i32>,
    ) -> Result<DnsDomainResponse, ShodanError>;

    async fn dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<HashMap<String, Option<String>>, ShodanError>;

    async fn dns_reverse(
        &self,
        ips: Vec<String>,
    ) -> Result<HashMap<String, Vec<String>>, ShodanError>;
}

#[async_trait]
impl Dns for ShodanClient {
    async fn dns_domain(
        &self,
        domain: String,
        history: Option<bool>,
        dns_type: Option<String>,
        page: Option<i32>,
    ) -> Result<DnsDomainResponse, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("history", history, &mut parameters);
        add_optional_parameter("dns_type", dns_type, &mut parameters);
        add_optional_parameter("page", page, &mut parameters);

        Self::fetch(
            self.build_request_url(format!("/dns/domain/{domain}").as_str(), Some(parameters)),
        )
        .await
    }

    async fn dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<HashMap<String, Option<String>>, ShodanError> {
        let parameters = HashMap::from([(String::from("hostnames"), hostnames.join(","))]);

        Self::fetch(self.build_request_url("/dns/resolve", Some(parameters))).await
    }

    async fn dns_reverse(
        &self,
        ips: Vec<String>,
    ) -> Result<HashMap<String, Vec<String>>, ShodanError> {
        let parameters = HashMap::from([(String::from("ips"), ips.join(","))]);

        Self::fetch(self.build_request_url("/dns/reverse", Some(parameters))).await
    }
}

#[cfg(test)]
pub mod tests {
    use crate::dns::Dns;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[tokio::test]
    async fn can_get_dns_domain() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .dns_domain(String::from("google.com"), None, None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_dns_resolve() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .dns_resolve(vec![
                String::from("google.com"),
                String::from("facebook.com"),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_dns_reverse() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .dns_reverse(vec![String::from("8.8.8.8"), String::from("1.1.1.1")])
            .await
            .unwrap();
    }
}
