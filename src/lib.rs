use crate::response::ShodanClientResponse;
use std::collections::HashMap;
use url::Url;

pub mod account;
pub mod api_info;
pub mod response;

const BASE_API_URL: &'static str = "https://api.shodan.io";

pub struct ShodanClient {
    api_key: &'static str,
}

impl ShodanClient {
    pub fn new(api_key: &'static str) -> Self {
        Self { api_key }
    }

    fn build_request_url(
        &self,
        endpoint: &str,
        parameters: Option<HashMap<String, String>>,
    ) -> String {
        let mut url = Url::parse(BASE_API_URL).unwrap();
        url.set_path(endpoint);

        url.query_pairs_mut().append_pair("key", self.api_key);

        if let Some(url_parameters) = parameters {
            url.query_pairs_mut()
                .extend_pairs(url_parameters.into_iter());
        }

        return url.to_string();
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    pub fn get_test_api_key() -> &'static str {
        let api_key = env::var("SHODAN_TEST_KEY");
        match api_key {
            // Unit tests aren't complete without the necessary memory leak
            Ok(key) => Box::leak(key.into_boxed_str()),
            Err(_) => panic!("Did not specify a shodan API key for testing"),
        }
    }
}
