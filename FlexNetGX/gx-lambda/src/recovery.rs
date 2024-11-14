use std::time::Duration;
use tokio::time::sleep;
use crate::error::LambdaError;

pub struct RetryConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(10),
            backoff_factor: 2.0,
        }
    }
}

pub async fn with_retry<F, Fut, T>(
    operation: F,
    config: RetryConfig,
) -> Result<T, LambdaError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, LambdaError>>,
{
    let mut current_delay = config.initial_delay;
    let mut attempts = 0;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempts += 1;
                if attempts >= config.max_attempts {
                    return Err(e);
                }

                if should_retry(&e) {
                    sleep(current_delay).await;
                    current_delay = std::cmp::min(
                        current_delay.mul_f64(config.backoff_factor),
                        config.max_delay,
                    );
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

fn should_retry(error: &LambdaError) -> bool {
    match error {
        LambdaError::BlockchainError(_) => true,
        LambdaError::DatabaseError(_) => true,
        LambdaError::TransactionError(_) => true,
        LambdaError::ValidationError(_) => false,
    }
}