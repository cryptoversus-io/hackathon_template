use std::time::Duration;
use tokio::time::sleep;
use crate::db::DynamoDBClient;
use crate::blockchain::client::SolanaClient;

pub struct TransactionMonitor {
    solana: SolanaClient,
    db: DynamoDBClient,
    max_attempts: u32,
    delay: Duration,
}

impl TransactionMonitor {
    pub fn new(solana: SolanaClient, db: DynamoDBClient) -> Self {
        Self {
            solana,
            db,
            max_attempts: 10,
            delay: Duration::from_secs(2),
        }
    }

    pub async fn monitor_transaction(
        &self,
        signature: &str,
        bounty_id: &str,
    ) -> Result<bool> {
        for _ in 0..self.max_attempts {
            match self.solana.monitor_transaction(signature).await {
                Ok(true) => {
                    // Transaction confirmed, update status
                    self.db.update_transaction_status(bounty_id, signature, "confirmed").await?;
                    return Ok(true);
                }
                Ok(false) => {
                    sleep(self.delay).await;
                    continue;
                }
                Err(e) => {
                    self.db.update_transaction_status(bounty_id, signature, "failed").await?;
                    return Err(e);
                }
            }
        }

        self.db.update_transaction_status(bounty_id, signature, "timeout").await?;
        Ok(false)
    }
}