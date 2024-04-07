use crate::*;
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait Directory {
    async fn directory_query(
        &self,
        page: Option<u32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, Error>;

    async fn directory_query_search(
        &self,
        query: String,
        page: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, Error>;

    async fn directory_query_tags(
        &self,
        size: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, Error>;
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
        page: Option<u32>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("page", page);
        parameters.set_optional("sort", sort);
        parameters.set_optional("order", order);

        Self::fetch(self.build_request_url("/shodan/query", &parameters)?).await
    }

    async fn directory_query_search(
        &self,
        query: String,
        page: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryResponse>, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set("query", query);
        parameters.set_optional("page", page);

        Self::fetch(self.build_request_url("/shodan/query/search", &parameters)?).await
    }

    async fn directory_query_tags(
        &self,
        size: Option<u32>,
    ) -> Result<ShodanClientResponse<DirectoryQueryTagsResponse>, Error> {
        let mut parameters = ParameterBag::default();
        parameters.set_optional("size", size);

        Self::fetch(self.build_request_url("/shodan/query/tags", &parameters)?).await
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
