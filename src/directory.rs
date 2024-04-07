use crate::error::ShodanError;
use crate::response::ShodanClientResponse;
use crate::{add_optional_parameter, ShodanClient};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[async_trait]
pub trait Directory {
    async fn directory_query(
        &self,
        page: Option<i32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError>;

    async fn directory_query_search(
        &self,
        query: String,
        page: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError>;

    async fn directory_query_tags(
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

#[async_trait]
impl Directory for ShodanClient {
    async fn directory_query(
        &self,
        page: Option<i32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("page", page, &mut parameters);
        add_optional_parameter("sort", sort, &mut parameters);
        add_optional_parameter("order", order, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/query", Some(parameters))).await
    }

    async fn directory_query_search(
        &self,
        query: String,
        page: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, ShodanError> {
        let mut parameters = HashMap::from([(String::from("query"), query)]);
        add_optional_parameter("page", page, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/query/search", Some(parameters))).await
    }

    async fn directory_query_tags(
        &self,
        size: Option<i32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, ShodanError> {
        let mut parameters = HashMap::new();
        add_optional_parameter("size", size, &mut parameters);

        Self::fetch(self.build_request_url("/shodan/query/tags", None)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_test_api_key;
    use crate::*;

    #[tokio::test]
    async fn can_get_directory_query() {
        let client = ShodanClient::new(get_test_api_key());
        client.directory_query(None, None, None).await.unwrap();
    }

    #[tokio::test]
    async fn can_get_directory_query_search() {
        let client = ShodanClient::new(get_test_api_key());
        client
            .directory_query_search(String::from("webcam"), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_get_directory_query_tags() {
        let client = ShodanClient::new(get_test_api_key());
        client.directory_query_tags(None).await.unwrap();
    }
}
