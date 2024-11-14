// FlexNetGX/gx-web/src/services/moderate.rs
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::types::{moderateData, bounty, Analysis, bountyResponse};
use crate::config::API_BASE_URL;

pub struct moderateService {
    base_url: String,
}

impl moderateService {
    pub fn new() -> Self {
        Self {
            base_url: format!("{}/moderate", API_BASE_URL),
        }
    }

    pub async fn fetch_moderate_data(&self) -> Result<moderateData, String> {
        let request = self.create_request("GET", "data", None)?;
        let data = self.send_request::<moderateData>(request).await?;
        Ok(data)
    }

    pub async fn create_bounty(&self, bounty: bounty) -> Result<bounty, String> {
        let body = serde_json::to_string(&bounty)
            .map_err(|e| e.to_string())?;
        
        let request = self.create_request("POST", "bounties", Some(&body))?;
        let created_bounty = self.send_request::<bounty>(request).await?;
        Ok(created_bounty)
    }

    pub async fn update_analysis(&self, analysis: Analysis) -> Result<Analysis, String> {
        let body = serde_json::to_string(&analysis)
            .map_err(|e| e.to_string())?;
        
        let request = self.create_request(
            "PUT", 
            &format!("analysis/{}", analysis.id), 
            Some(&body)
        )?;
        
        let updated_analysis = self.send_request::<Analysis>(request).await?;
        Ok(updated_analysis)
    }

    pub async fn export_data(&self, format: &str) -> Result<Vec<u8>, String> {
        let request = self.create_request(
            "GET", 
            &format!("export?format={}", format), 
            None
        )?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|err| err.as_string().unwrap_or_else(|| "Failed to fetch".to_string()))?;

        let resp: Response = resp_value.dyn_into().unwrap();
        
        let array_buffer = JsFuture::from(resp.array_buffer()?)
            .await
            .map_err(|_| "Failed to get array buffer".to_string())?;
        
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let mut result = vec![0; uint8_array.length() as usize];
        uint8_array.copy_to(&mut result);
        
        Ok(result)
    }

    // Helper methods
    fn create_request(&self, method: &str, path: &str, body: Option<&str>) -> Result<Request, String> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        if let Some(body) = body {
            opts.body(Some(&JsValue::from_str(body)));
        }

        let url = format!("{}/{}", self.base_url, path);
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|err| err.as_string().unwrap_or_else(|| "Failed to create request".to_string()))?;

        request.headers().set("Authorization", &self.get_auth_header()?)?;
        request.headers().set("Content-Type", "application/json")?;

        Ok(request)
    }

    async fn send_request<T>(&self, request: Request) -> Result<T, String> 
    where
        T: for<'de> serde::Deserialize<'de>
    {
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|err| err.as_string().unwrap_or_else(|| "Failed to fetch".to_string()))?;

        let resp: Response = resp_value.dyn_into().unwrap();

        if !resp.ok() {
            return Err(format!("HTTP error: {}", resp.status()));
        }

        let json = JsFuture::from(resp.json()?)
            .await
            .map_err(|err| err.as_string().unwrap_or_else(|| "Failed to parse response".to_string()))?;

        let data: T = serde_wasm_bindgen::from_value(json)?;
        Ok(data)
    }
}