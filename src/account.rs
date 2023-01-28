use crate::ShodanClient;
use serde::Deserialize;
use async_trait::async_trait;
use crate::error::ShodanError;

#[async_trait]
pub trait Account {
    async fn get_account_profile(&self) -> Result<AccountProfileResponse, ShodanError>;
}

#[derive(Deserialize, Debug)]
pub struct AccountProfileResponse {
    pub member: bool,
    pub credits: u32,
    pub display_name: Option<String>,
    pub created: String,
}

#[async_trait]
impl Account for ShodanClient {
    async fn get_account_profile(&self) -> Result<AccountProfileResponse, ShodanError> {
        Self::fetch(self.build_request_url("/account/profile", None)).await
    }
}

#[cfg(test)]
pub mod tests {
    use crate::account::Account;
    use crate::ShodanClient;
    use crate::tests::get_test_api_key;

    #[tokio::test]
    async fn can_get_account_profile() {
        let client = ShodanClient::new(get_test_api_key());
        client.get_account_profile().await.unwrap();
    }
}
