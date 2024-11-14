use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use sha2::{Sha256, Digest};
use blake3;
use rusoto_core::Region;
use rusoto_kms::KmsClient;

mod models;
mod blockchain;
mod handlers;

use crate::models::{BountyRequest, SubmissionRequest};
use crate::handlers::{handle_create_bounty, handle_submission};
use crate::recovery::{with_retry, RetryConfig};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let retry_config = RetryConfig::default();

    let result = with_retry(
        || async {
            // Your handler logic here
            let (event, _context) = event.clone().into_parts();
            
            let action = event["action"]
                .as_str()
                .ok_or_else(|| LambdaError::ValidationError("Missing action".to_string()))?;
            
            match action {
                "create_bounty" => handle_create_bounty(event["body"].clone()).await?,
                "list_bounties" => handle_list_bounties().await?,
                "submit_work" => handle_submit_work(event["body"].clone()).await?,
                "get_bounty" => handle_get_bounty(event["body"].clone()).await?,
                _ => return Err(LambdaError::ValidationError("Invalid action".to_string())),
            }
        },
        retry_config,
    ).await?;

    Ok(json!({
        "statusCode": 200,
        "body": result
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    
    let action = event["action"]
        .as_str()
        .ok_or_else(|| Error::from("Missing action field"))?;
    
    let response = match action {
        "create_bounty" => handle_create_bounty(event["body"].clone()).await?,
        "list_bounties" => handle_list_bounties().await?,
        "submit_work" => handle_submit_work(event["body"].clone()).await?,
        "get_bounty" => handle_get_bounty(event["body"].clone()).await?,
        _ => return Ok(json!({
            "statusCode": 400,
            "body": json!({
                "error": "Invalid action"
            })
        }))
    };

    Ok(json!({
        "statusCode": 200,
        "body": response
    }))
}
    
    // Extract action from API Gateway event
    let action = event["action"]
        .as_str()
        .ok_or_else(|| Error::from("Missing action field"))?;

    let response = match action {
        "create_bounty" => {
            let request: BountyRequest = serde_json::from_value(event["body"].clone())?;
            handle_create_bounty(request).await?
        },
        "submit_work" => {
            let request: SubmissionRequest = serde_json::from_value(event["body"].clone())?;
            handle_submission(request).await?
        },
        _ => return Ok(json!({
            "statusCode": 400,
            "body": json!({
                "error": "Invalid action"
            })
        }))
    };

    Ok(json!({
        "statusCode": 200,
        "body": response
    }))
}
