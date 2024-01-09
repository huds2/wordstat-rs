use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;

pub struct ReportRequest {
    phrases: Vec<String>,
    geo_id: Vec<i64>
}

impl ReportRequest {
    pub fn new() -> Self {
        ReportRequest { phrases: vec![], geo_id: vec![] }
    }
    pub fn add_phrase(mut self, phrase: &str) -> Self {
        self.phrases.push(phrase.to_string());
        self
    }
    pub fn with_phrases(mut self, phrases: &Vec<&str>) -> Self {
        self.phrases = phrases.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn add_geo(mut self, geo_id: i64) -> Self {
        self.geo_id.push(geo_id);
        self
    }
    pub fn with_geo(mut self, geo_ids: &Vec<i64>) -> Self {
        self.geo_id = geo_ids.clone();
        self
    }
}

pub async fn create_report(client: &Client, request: &ReportRequest) -> Result<i64, WordstatError> {
    let method = "CreateNewWordstatReport";
    let mut params = serde_json::Map::new();
    params.insert("Phrases".to_string(), Value::from(request.phrases.clone()));
    params.insert("GeoID".to_string(), Value::from(request.geo_id.clone()));
    let result = client.post(method, Some(params.into())).await?;

    check_status(&result)?;

    let Some(data) = result.get("data") else { return Err(WordstatError::BadResponse) };
    let Value::Number(report_id) = data else { return Err(WordstatError::BadResponse) };
    if !report_id.is_i64() { return Err(WordstatError::BadResponse) }

    Ok(report_id.as_i64().unwrap() as i64)
}

// TODO write unit tests
