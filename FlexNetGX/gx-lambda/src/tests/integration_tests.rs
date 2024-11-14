// gx-lambda/tests/integration_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_bounty() {
        let request = BountyRequest {
            description: "Test bounty".to_string(),
            amount: 1000,
            creator_pubkey: "test_pubkey".to_string(),
        };

        let response = handle_create_bounty(request).await.unwrap();
        assert_eq!(response.status, "success");
    }
}