// FlexNetGX/gx-web/src/services/bountyhunter.rs
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode};
use crate::types::{bounty, Badge, Task, bountyhunterData};

#[derive(Debug, Serialize)]
struct bountyResponse {
    bounty_id: String,
    responses: Vec<String>,
    timestamp: String,
}

pub struct bountyhunterService {
    base_url: String,
}

impl bountyhunterService {
    pub fn new() -> Self {
        Self {
            base_url: format!("{}/bountyhunter", API_BASE_URL),
        }
    }

    pub async fn fetch_bountyhunter_data(&self) -> Result<bountyhunterData, String> {
        let request = self.create_request("GET", "data", None)?;
        let data = self.send_request::<bountyhunterData>(request).await?;
        Ok(data)
    }

    pub async fn submit_bounty_response(
        &self,
        bounty_id: String,
        responses: Vec<String>
    ) -> Result<(), String> {
        let response = bountyResponse {
            bounty_id,
            responses,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let body = serde_json::to_string(&response)
            .map_err(|e| e.to_string())?;

        let request = self.create_request("POST", "bounties/submit", Some(&body))?;
        self.send_request::<()>(request).await
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), String> {
        let request = self.create_request(
            "POST", 
            &format!("tasks/{}/complete", task_id),
            None
        )?;
        self.send_request::<()>(request).await
    }

    pub async fn claim_badge(&self, badge_id: &str) -> Result<Badge, String> {
        let request = self.create_request(
            "POST",
            &format!("badges/{}/claim", badge_id),
            None
        )?;
        self.send_request::<Badge>(request).await
    }

    // Helper methods for request handling
    async fn send_request<T>(&self, request: Request) -> Result<T, String>
    where
        T: for<'de> serde::Deserialize<'de>
    {
        let window = web_sys::window().unwrap();
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("Failed to fetch: {:?}", e))?;

        let response: web_sys::Response = response.dyn_into()
            .map_err(|_| "Failed to convert response".to_string())?;

        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let json = JsFuture::from(response.json()?)
            .await
            .map_err(|_| "Failed to parse JSON".to_string())?;

        let result = serde_wasm_bindgen::from_value(json)
            .map_err(|e| format!("Failed to deserialize: {:?}", e))?;

        Ok(result)
    }

    fn create_request(&self, method: &str, path: &str, body: Option<&str>) -> Result<Request, String> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        if let Some(body) = body {
            opts.body(Some(&wasm_bindgen::JsValue::from_str(body)));
        }

        let url = format!("{}/{}", self.base_url, path);
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| "Failed to create request".to_string())?;

        let headers = request.headers();
        headers.set("Content-Type", "application/json")?;
        
        // Add authorization header
        if let Some(token) = self.get_auth_token() {
            headers.set("Authorization", &format!("Bearer {}", token))?;
        }

        Ok(request)
    }

    fn get_auth_token(&self) -> Option<String> {
        // Implement token retrieval from local storage or state management
        None
    }
}