// gx-lambda/src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BountyAccount {
    pub id: String,
    pub creator: String,
    pub amount: u64,
    pub description_hash: [u8; 32],
    pub status: BountyStatus,
    pub hunter: Option<String>,
}

#[derive(Deserialize)]
pub struct BountyRequest {
    pub description: String,
    pub amount: u64,
    pub creator_pubkey: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bounty {
    pub id: String,
    pub description: String,
    pub amount: u64,
    pub status: BountyStatus,
    pub creator: String,
}

#[derive(Serialize)]
pub struct BountyResponse {
    pub status: String,
    pub transaction_signature: Option<String>,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SubmissionRequest {
    pub bounty_pubkey: String,
    pub submission_data: String,
    pub hunter_pubkey: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BountyStatus {
    Open,
    InProgress,
    Completed,
}