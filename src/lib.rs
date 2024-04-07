use serde::Deserialize;
use std::collections::{hash_map, HashMap};
use url::Url;

mod account;
mod api_status;
mod builders;
mod directory;
mod dns;
mod error;
mod response;
mod scanning;
mod search;
mod utility;

pub use account::*;
pub use api_status::*;
pub use builders::*;
pub use directory::*;
pub use dns::*;
pub use error::*;
pub use response::*;
pub use scanning::*;
pub use search::*;
pub use utility::*;

const BASE_API_URL: &str = "https://api.shodan.io";

pub struct ShodanClient {
    api_key: String,
}

impl ShodanClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn build_request_url(&self, endpoint: &str, parameters: &ParameterBag) -> Result<String, error::Error> {
        let mut url = Url::parse(BASE_API_URL)?;
        url.set_path(endpoint);

        // Set API key
        url.query_pairs_mut()
            .append_pair("key", self.api_key.as_str());

        // Set any additional parameters
        url.query_pairs_mut().extend_pairs(parameters.pairs());

        Ok(url.to_string())
    }

    async fn fetch<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, Error> {
        let response = reqwest::get(url)
            .await?
            .json::<ShodanClientResponse<T>>()
            .await?;

        match response {
            ShodanClientResponse::Error(e) => {
                Err(error::Error::Shodan(format!("Error response: {}", e.error)))
            },
            ShodanClientResponse::Response(r) => Ok(r),
        }
    }
}

#[derive(Default)]
struct ParameterBag(HashMap<String, String>);

impl ParameterBag {
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn set_optional(
        &mut self,
        key: impl Into<String>,
        value: Option<impl Into<ParameterValue>>,
    ) {
        if let Some(value) = value {
            self.set(key.into(), value.into().0);
        }
    }

    pub fn pairs(&self) -> hash_map::Iter<'_, String, String> {
        self.0.iter()
    }
}

pub struct ParameterValue(String);

impl From<u32> for ParameterValue {
    fn from(value: u32) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ParameterValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ParameterValue {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<bool> for ParameterValue {
    fn from(value: bool) -> Self {
        Self(if value { "true".into() } else { "false".into() })
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
