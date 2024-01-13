use reqwest::StatusCode;
use serde_json::Value;
use crate::WordstatError;

pub struct Client {
    token: String,
    api_url: String,
    client: reqwest::Client
}

#[cfg_attr(test, mockall::automock)]
impl Client {
    pub fn new(token: &str, api_url: &str) -> Self {
        Client { 
            token: token.to_string(),
            api_url: api_url.to_string(),
            client: reqwest::Client::new()
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    pub fn set_url(&mut self, api_url: &str) {
        self.api_url = api_url.to_string();
    }

    pub async fn post(&self, method: &str, params: Option<Value>) -> Result<serde_json::Value, WordstatError> {
        let mut payload = serde_json::Map::new();
        payload.insert("method".to_string(), Value::from(method));
        payload.insert("token".to_string(), Value::from(self.token.as_str()));
        if let Some(param) = params {
            payload.insert("param".to_string(), param);
        }

        let response = self.client.post(self.api_url.as_str())
            .json(&payload)
            .send()
            .await.unwrap();
        if response.status() != StatusCode::OK {
            return Err(WordstatError::UnknownResponseCode { code: response.status().as_u16() as i64 });
        }

        let Ok(response_text) = response.text().await else { return Err(WordstatError::UnknownError) };
        let Ok(response_json): Result<Value, serde_json::Error> = 
                               serde_json::from_str(&response_text) else { return Err(WordstatError::BadResponse{ reason: "Failed to read JSON response" }) };

        Ok(response_json)
    }
}
