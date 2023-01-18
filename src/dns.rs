use crate::response::ShodanClientResponse;
use crate::ShodanClient;
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

trait Dns {
    fn get_dns_domain(
        &self,
        domain: String,
    ) -> Result<ShodanClientResponse<DnsDomainResponse>, reqwest::Error>;

    fn get_dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<ShodanClientResponse<HashMap<String, Option<String>>>, reqwest::Error>;

    fn get_dns_reverse(
        &self,
        ips: Vec<String>,
    ) -> Result<ShodanClientResponse<HashMap<String, Vec<String>>>, reqwest::Error>;
}

impl Dns for ShodanClient {
    fn get_dns_domain(
        &self,
        domain: String,
    ) -> Result<ShodanClientResponse<DnsDomainResponse>, reqwest::Error> {
        Self::fetch(self.build_request_url(format!("/dns/domain/{domain}").as_str(), None))
    }

    fn get_dns_resolve(
        &self,
        hostnames: Vec<String>,
    ) -> Result<ShodanClientResponse<HashMap<String, Option<String>>>, reqwest::Error> {
        let parameters = HashMap::from([(String::from("hostnames"), hostnames.join(","))]);

        Self::fetch(self.build_request_url("/dns/resolve", Some(parameters)))
    }

    fn get_dns_reverse(
        &self,
        ips: Vec<String>,
    ) -> Result<ShodanClientResponse<HashMap<String, Vec<String>>>, reqwest::Error> {
        let parameters = HashMap::from([(String::from("ips"), ips.join(","))]);

        Self::fetch(self.build_request_url("/dns/reverse", Some(parameters)))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::dns::Dns;
    use crate::response::ShodanClientResponse;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_dns_domain() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_dns_domain(String::from("google.com")).unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_dns_resolve() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .get_dns_resolve(vec![
                String::from("google.com"),
                String::from("facebook.com"),
            ])
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }

    #[test]
    fn can_get_dns_reverse() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .get_dns_reverse(vec![String::from("8.8.8.8"), String::from("1.1.1.1")])
            .unwrap();

        assert!(
            matches!(response, ShodanClientResponse::Response { .. }),
            "Response was {:?}",
            response
        );
    }
}
