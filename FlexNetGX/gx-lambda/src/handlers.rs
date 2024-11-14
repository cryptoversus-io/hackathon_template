// gx-lambda/src/handlers.rs
use lambda_runtime::{Error, LambdaEvent};
use serde_json::{json, Value};
use crate::models::{BountyRequest, BountyResponse, SubmissionRequest};
use crate::blockchain::BlockchainService;
use crate::blockchain::SolanaClient;
use crate::db::DynamoDBClient;
use serde_json::Value;
use anyhow::Result;

pub async fn handle_create_bounty(event: BountyRequest) -> Result<BountyResponse, Error> {
    let blockchain = BlockchainService::new();
    let description_hash = blockchain.hash_description(&event.description);

    // MVP: Log the request details
    println!("Creating bounty: Amount={}, Creator={}", event.amount, event.creator_pubkey);

    Ok(BountyResponse {
        status: "success".to_string(),
        transaction_signature: Some("simulation_sig".to_string()),
        message: "Bounty created successfully".to_string(),
    })
}

pub async fn handle_submission(event: SubmissionRequest) -> Result<BountyResponse, Error> {
    let blockchain = BlockchainService::new();
    let submission_hash = blockchain.hash_description(&event.submission_data);

    // MVP: Log the submission details
    println!("Processing submission for bounty: {}", event.bounty_pubkey);

    Ok(BountyResponse {
        status: "success".to_string(),
        transaction_signature: Some("submission_sig".to_string()),
        message: "Submission processed successfully".to_string(),
    })
}

pub async fn handle_create_bounty(payload: Value) -> Result<Value> {
    let db = DynamoDBClient::new();
    let solana = SolanaClient::new();

    let title = payload["title"].as_str().ok_or_else(|| anyhow::anyhow!("Missing title"))?;
    let description = payload["description"].as_str().ok_or_else(|| anyhow::anyhow!("Missing description"))?;
    let amount = payload["amount"].as_u64().ok_or_else(|| anyhow::anyhow!("Missing amount"))?;
    let creator_pubkey = payload["creator_pubkey"].as_str().ok_or_else(|| anyhow::anyhow!("Missing creator_pubkey"))?;

    // Create blockchain transaction
    let transaction_id = solana.create_bounty(creator_pubkey, amount).await?;

    // Store bounty data
    let bounty_id = uuid::Uuid::new_v4().to_string();
    let bounty = json!({
        "id": bounty_id,
        "title": title,
        "description": description,
        "amount": amount,
        "creator_pubkey": creator_pubkey,
        "status": "Open",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "transaction_id": transaction_id
    });

    db.put_bounty(&bounty).await?;

    Ok(json!({
        "bounty": bounty,
        "transaction_id": transaction_id
    }))
}

pub async fn handle_list_bounties() -> Result<Value> {
    let db = DynamoDBClient::new();
    let bounties = db.list_bounties().await?;
    Ok(json!(bounties))
}

pub async fn handle_submit_work(payload: Value) -> Result<Value> {
    let db = DynamoDBClient::new();
    let solana = SolanaClient::new();

    let bounty_id = payload["bounty_id"].as_str().ok_or_else(|| anyhow::anyhow!("Missing bounty_id"))?;
    let submission_data = payload["submission_data"].as_str().ok_or_else(|| anyhow::anyhow!("Missing submission_data"))?;
    let hunter_pubkey = payload["hunter_pubkey"].as_str().ok_or_else(|| anyhow::anyhow!("Missing hunter_pubkey"))?;

    // Create blockchain transaction for submission
    let transaction_id = solana.submit_work(bounty_id, hunter_pubkey).await?;

    // Store submission data
    let submission = json!({
        "id": uuid::Uuid::new_v4().to_string(),
        "bounty_id": bounty_id,
        "hunter_pubkey": hunter_pubkey,
        "content": submission_data,
        "status": "Pending",
        "submitted_at": chrono::Utc::now().to_rfc3339(),
        "transaction_id": transaction_id
    });

    db.put_submission(&submission).await?;

    // Update bounty status
    db.update_bounty_status(bounty_id, "InProgress").await?;

    Ok(json!({
        "submission": submission,
        "transaction_id": transaction_id
    }))
}

pub async fn handle_get_bounty(payload: Value) -> Result<Value> {
    let db = DynamoDBClient::new();
    let bounty_id = payload["bounty_id"].as_str().ok_or_else(|| anyhow::anyhow!("Missing bounty_id"))?;
    
    let bounty = db.get_bounty(bounty_id).await?;
    Ok(json!(bounty))
}