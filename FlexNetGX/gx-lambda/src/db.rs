use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, GetItemInput, QueryInput};
use serde_json::Value;
use anyhow::Result;
use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, PutItemInput, GetItemInput, QueryInput, 
    UpdateItemInput, AttributeValue, PutItemError
};
use serde_json::{Value, json};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

pub struct DynamoDBClient {
    client: DynamoDbClient,
    table_name: String,
}

impl DynamoDBClient {
    pub fn new() -> Self {
        Self {
            client: DynamoDbClient::new(Region::UsEast1),
            table_name: std::env::var("DYNAMODB_TABLE")
                .unwrap_or_else(|_| "flexnetgx-bounties".to_string()),
        }
    }


    pub async fn put_bounty(&self, bounty: &Value) -> Result<()> {
        let mut item = HashMap::new();
        
        item.insert(
            "PK".to_string(),
            AttributeValue {
                s: Some(format!("BOUNTY#{}", bounty["id"].as_str().unwrap())),
                ..Default::default()
            },
        );
        
        item.insert(
            "SK".to_string(),
            AttributeValue {
                s: Some("METADATA".to_string()),
                ..Default::default()
            },
        );

        item.insert(
            "data".to_string(),
            AttributeValue {
                s: Some(serde_json::to_string(bounty)?),
                ..Default::default()
            },
        );

        let input = PutItemInput {
            table_name: self.table_name.clone(),
            item,
            ..Default::default()
        };

        self.client.put_item(input).await
            .map_err(|e| anyhow!("Failed to put bounty: {}", e))?;

        Ok(())
    }


    pub async fn list_bounties(&self) -> Result<Vec<Value>> {
        let input = QueryInput {
            table_name: self.table_name.clone(),
            key_condition_expression: Some("begins_with(PK, :prefix)".to_string()),
            expression_attribute_values: Some(HashMap::from([(
                ":prefix".to_string(),
                AttributeValue {
                    s: Some("BOUNTY#".to_string()),
                    ..Default::default()
                },
            )])),
            ..Default::default()
        };

        let result = self.client.query(input).await
            .map_err(|e| anyhow!("Failed to list bounties: {}", e))?;

        let mut bounties = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(data) = item.get("data") {
                    if let Some(json_str) = &data.s {
                        bounties.push(serde_json::from_str(json_str)?);
                    }
                }
            }
        }

        Ok(bounties)
    }

    pub async fn get_bounty(&self, id: &str) -> Result<Value> {
        let mut key = HashMap::new();
        
        key.insert(
            "PK".to_string(),
            AttributeValue {
                s: Some(format!("BOUNTY#{}", id)),
                ..Default::default()
            },
        );
        
        key.insert(
            "SK".to_string(),
            AttributeValue {
                s: Some("METADATA".to_string()),
                ..Default::default()
            },
        );

        let input = GetItemInput {
            table_name: self.table_name.clone(),
            key,
            ..Default::default()
        };

        let result = self.client.get_item(input).await
            .map_err(|e| anyhow!("Failed to get bounty: {}", e))?;

        if let Some(item) = result.item {
            if let Some(data) = item.get("data") {
                if let Some(json_str) = &data.s {
                    return Ok(serde_json::from_str(json_str)?);
                }
            }
        }

        Err(anyhow!("Bounty not found"))
    }

    pub async fn update_bounty_status(&self, id: &str, status: &str) -> Result<()> {
        let mut key = HashMap::new();
        key.insert(
            "PK".to_string(),
            AttributeValue {
                s: Some(format!("BOUNTY#{}", id)),
                ..Default::default()
            },
        );
        key.insert(
            "SK".to_string(),
            AttributeValue {
                s: Some("METADATA".to_string()),
                ..Default::default()
            },
        );

        let input = UpdateItemInput {
            table_name: self.table_name.clone(),
            key,
            update_expression: Some("SET #status = :status".to_string()),
            expression_attribute_names: Some(HashMap::from([
                ("#status".to_string(), "status".to_string()),
            ])),
            expression_attribute_values: Some(HashMap::from([
                (":status".to_string(), AttributeValue {
                    s: Some(status.to_string()),
                    ..Default::default()
                }),
            ])),
            ..Default::default()
        };

        self.client.update_item(input).await
            .map_err(|e| anyhow!("Failed to update bounty status: {}", e))?;

        Ok(())
    }
}

    pub async fn put_submission(&self, submission: &Value) -> Result<()> {
        // Implement DynamoDB put_item for submissions
        Ok(())
    }
}