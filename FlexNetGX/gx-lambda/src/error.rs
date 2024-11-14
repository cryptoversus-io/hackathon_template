use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum LambdaError {
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

impl fmt::Display for LambdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LambdaError::BlockchainError(msg) => write!(f, "Blockchain error: {}", msg),
            LambdaError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            LambdaError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            LambdaError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
}
