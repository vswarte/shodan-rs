use crate::error::ShodanError::ShodanClientError;
use serde::Deserialize;
use std::collections::HashMap;
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
            url.query_pairs_mut().extend_pairs(url_parameters);
        }

        url.to_string()
    }

    async fn fetch<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, ShodanError> {
        let response = reqwest::get(url)
            .await
            .map_err(ShodanError::ReqwestError)?
            .json::<ShodanClientResponse<T>>()
            .await
            .map_err(ShodanError::ReqwestError)?;

        match response {
            ShodanClientResponse::Error(e) => {
                Err(ShodanClientError(format!("Error response: {}", e.error)))
            }
            ShodanClientResponse::Response(r) => Ok(r),
        }
    }
}

pub fn add_optional_parameter(
    name: &str,
    param: Option<impl ToString>,
    map: &mut HashMap<String, String>,
) {
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
