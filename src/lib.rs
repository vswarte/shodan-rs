use crate::response::ShodanClientResponse;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

pub mod account;
pub mod api_status;
pub mod directory;
pub mod dns;
pub mod response;
pub mod scanning;
pub mod search;
pub mod utility;

const BASE_API_URL: &'static str = "https://api.shodan.io";

pub struct ShodanClient {
    api_key: String,
}

impl ShodanClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn build_request_url(
        &self,
        endpoint: &str,
        parameters: Option<HashMap<String, String>>,
    ) -> String {
        let mut url = Url::parse(BASE_API_URL).unwrap();
        url.set_path(endpoint);

        url.query_pairs_mut()
            .append_pair("key", self.api_key.as_str());

        if let Some(url_parameters) = parameters {
            url.query_pairs_mut()
                .extend_pairs(url_parameters.into_iter());
        }

        return url.to_string();
    }

    fn fetch<T: for<'a> Deserialize<'a>>(
        url: String,
    ) -> Result<ShodanClientResponse<T>, reqwest::Error> {
        Ok(reqwest::blocking::get(url)?.json::<ShodanClientResponse<T>>()?)
    }
}

pub fn add_parameter(name: &str, param: Option<impl ToString>, map: &mut HashMap<String, String>) {
    if let Some(unwrapped) = param {
        map.insert(name.into(), unwrapped.to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    pub fn get_test_api_key() -> String {
        let api_key = env::var("SHODAN_TEST_KEY");
        match api_key {
            Ok(key) => key,
            Err(_) => panic!("Did not specify a shodan API key for testing"),
        }
    }
}
