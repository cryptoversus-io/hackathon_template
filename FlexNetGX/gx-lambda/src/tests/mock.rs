use async_trait::async_trait;
use mockall::mock;
use crate::blockchain::client::{SolanaClient, BountyAccount};
use crate::db::DynamoDBClient;

mock! {
    pub SolanaClientMock {
        pub async fn create_bounty(
            &self,
            amount: u64,
            description_hash: [u8; 32],
            creator: Pubkey,
        ) -> Result<(String, Pubkey)>;

        pub async fn submit_work(
            &self,
            bounty_pubkey: &Pubkey,
            submission_hash: [u8; 32],
            hunter: &Pubkey,
        ) -> Result<String>;

        pub async fn monitor_transaction(&self, signature: &str) -> Result<bool>;
    }
}

mock! {
    pub DynamoDBClientMock {
        pub async fn put_bounty(&self, bounty: &Value) -> Result<()>;
        pub async fn get_bounty(&self, id: &str) -> Result<Value>;
        pub async fn list_bounties(&self) -> Result<Vec<Value>>;
        pub async fn update_bounty_status(&self, id: &str, status: &str) -> Result<()>;
    }
}
