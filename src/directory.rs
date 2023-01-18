use crate::response::ShodanClientResponse;
use crate::ShodanClient;
use serde::Deserialize;
use std::collections::HashMap;

pub trait Directory {
    fn get_directory_query(
        &self,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, reqwest::Error>;

    fn get_directory_query_search(
        &self,
        query: String,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, reqwest::Error>;

    fn get_directory_query_tags(
        &self,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, reqwest::Error>;
}

#[derive(Deserialize, Debug)]
pub struct DirectoryQueryResponse {
    pub matches: Vec<DirectoryQueryResponseMatch>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct DirectoryQueryResponseMatch {
    pub votes: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub timestamp: String,
    pub title: String,
    pub query: String,
}

#[derive(Deserialize, Debug)]
pub struct DirectoryQueryTagsResponse {
    pub matches: Vec<DirectoryQueryTagsResponseMatch>,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct DirectoryQueryTagsResponseMatch {
    pub count: u32,
    pub value: String,
}

impl Directory for ShodanClient {
    fn get_directory_query(
        &self,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/query", None))
    }

    fn get_directory_query_search(
        &self,
        query: String,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, reqwest::Error> {
        let parameters = HashMap::from([(String::from("query"), query)]);

        Self::fetch(self.build_request_url("/shodan/query/search", Some(parameters)))
    }

    fn get_directory_query_tags(
        &self,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, reqwest::Error> {
        Self::fetch(self.build_request_url("/shodan/query/tags", None))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::directory::Directory;
    use crate::response::ShodanClientResponse;
    use crate::tests::get_test_api_key;
    use crate::ShodanClient;

    #[test]
    fn can_get_directory_query() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_directory_query().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }), "Response was {:?}", response);
    }

    #[test]
    fn can_get_directory_query_search() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .get_directory_query_search(String::from("webcam"))
            .unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }), "Response was {:?}", response);
    }

    #[test]
    fn can_get_directory_query_tags() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.get_directory_query_tags().unwrap();

        assert!(matches!(response, ShodanClientResponse::Response { .. }), "Response was {:?}", response);
    }
}
