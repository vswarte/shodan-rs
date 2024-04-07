use crate::*;
use async_trait::async_trait;
use serde::Deserialize;

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
        page: Option<u32>,
    ) -> Result<DnsDomainResponse, Error>;

    async fn dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<HashMap<String, Option<String>>, Error>;

    async fn dns_reverse(&self, ips: Vec<String>) -> Result<HashMap<String, Vec<String>>, Error>;
}

#[async_trait]
impl Dns for ShodanClient {
    async fn dns_domain(
        &self,
        domain: String,
        history: Option<bool>,
        dns_type: Option<String>,
        page: Option<u32>,
    ) -> Result<DnsDomainResponse, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("history", history);
        parameters.set_optional("dns_type", dns_type);
        parameters.set_optional("page", page);

        Self::fetch(self.build_request_url(format!("/dns/domain/{domain}").as_str(), &parameters)?)
            .await
    }

    async fn dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<HashMap<String, Option<String>>, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set("hostnames", hostnames.join(","));

        Self::fetch(self.build_request_url("/dns/resolve", &parameters)?).await
    }

    async fn dns_reverse(&self, ips: Vec<String>) -> Result<HashMap<String, Vec<String>>, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set("ips", ips.join(","));

        Self::fetch(self.build_request_url("/dns/reverse", &parameters)?).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

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
