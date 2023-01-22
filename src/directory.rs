use crate::error::ShodanError;
use crate::response::ShodanClientResponse;
use crate::{add_optional_parameter, ShodanClient};
use serde::Deserialize;
use std::collections::HashMap;

pub trait Directory {
    fn directory_query(
        &self,
        page: Option<i32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError>;

    fn directory_query_search(
        &self,
        query: String,
        page: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError>;

    fn directory_query_tags(
        &self,
        size: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, ShodanError>;
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
    fn directory_query(
        &self,
        page: Option<i32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("page", page, &mut parameters);
        add_optional_parameter("sort", sort, &mut parameters);
        add_optional_parameter("order", order, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/query", Some(parameters)))
    }

    fn directory_query_search(
        &self,
        query: String,
        page: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("page", page, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/query/search", Some(parameters)))
    }

    fn directory_query_tags(
        &self,
        size: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("size", size, &mut parameters);

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
        let response = client.directory_query(None, None, None).unwrap();
    }

    #[test]
    fn can_get_directory_query_search() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client
            .directory_query_search(String::from("webcam"), None)
            .unwrap();
    }

    #[test]
    fn can_get_directory_query_tags() {
        let client = ShodanClient::new(get_test_api_key());
        let response = client.directory_query_tags(None).unwrap();
    }
}
