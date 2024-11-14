use tokio;
use anyhow::Result;
use crate::handler;
use crate::tests::mock::{MockSolanaClient, MockDynamoDBClient};

#[tokio::test]
async fn test_create_bounty() -> Result<()> {
    let mut mock_solana = MockSolanaClient::new();
    let mut mock_db = MockDynamoDBClient::new();

    // Setup expectations
    mock_solana.expect_create_bounty()
        .returning(|_, _, _| {
            Ok((
                "test_signature".to_string(),
                Pubkey::new_unique(),
            ))
        });

    mock_db.expect_put_bounty()
        .returning(|_| Ok(()));

    let event = json!({
        "action": "create_bounty",
        "body": {
            "title": "Test Bounty",
            "description": "Test Description",
            "amount": 100,
            "creator_pubkey": "test_pubkey"
        }
    });

    let result = handler(LambdaEvent::new(event, Context::default())).await?;
    
    assert_eq!(result["statusCode"].as_i64().unwrap(), 200);
    Ok(())
}