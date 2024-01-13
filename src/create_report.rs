use serde_json::Value;
use crate::{WordstatError, check_status};
use crate::client::Client;

/// ReportRequest object is used to define the keywords
/// and regions used to get the statistics about keywords.
///
/// Can be constructed the following way:
/// ```
/// # use wordstat_rs::*;
/// let request = ReportRequest::new()
///     .add_phrase("rust lang").unwrap()
///     .add_geo(100);
/// ```
///
/// Geo is optional
pub struct ReportRequest {
    phrases: Vec<String>,
    geo_id: Vec<i64>
}

impl ReportRequest {
    /// Create a new ReportRequest object
    pub fn new() -> Self {
        ReportRequest { phrases: vec![], geo_id: vec![] }
    }
    /// Add phrases to ReportRequest
    /// Will return an Err if more than 10 phrases were supplied or
    /// an the phrase contains an character that is not allowed:
    ///
    /// Characters that are not allowed:
    /// - Plus sign '+'
    /// - Percent sign '%'
    /// - Ampresand '&'
    /// - Colon ':'
    /// - Minus sign with spaces on both sides ' - '
    ///
    /// To avoid searching a word you can prefix it with '-', like this:
    /// ```
    /// # use wordstat_rs::*;
    /// let request = ReportRequest::new()
    ///     .add_phrase("rust -steel").unwrap();
    /// ```
    /// To avoid searching for a phrase you can use parentheses like so:
    /// ```
    /// # use wordstat_rs::*;
    /// let request = ReportRequest::new()
    ///     .add_phrase("car -(diesel engine) repair)").unwrap();
    /// ```
    pub fn add_phrase(mut self, phrase: &str) -> Result<Self, WordstatError> {
        // API does not support more than 10 keyphrases in a single request
        if self.phrases.len() >= 10 { return Err(WordstatError::TooManyKeyphrases); }
        self.phrases.push(ReportRequest::check_phrase(phrase)?.to_string());
        Ok(self)
    }
    /// Pass a vector of phrases instead of inserting them one by one.
    /// Returns the same errors as [add_phrase](ReportRequest::add_phrase) method.
    pub fn with_phrases(mut self, phrases: &Vec<&str>) -> Result<Self, WordstatError> {
        for phrase in phrases {
            self = self.add_phrase(&phrase)?;
        }
        Ok(self)
    }
    /// Add region ID to be used when getting statistics.
    /// To get the list of regions use [get_regions](crate::region::get_regions) function.
    pub fn add_geo(mut self, geo_id: i64) -> Self {
        self.geo_id.push(geo_id);
        self
    }
    /// Same as [add_geo](ReportRequest::add_geo) but takes a vector of items instead of
    /// a single one.
    pub fn with_geo(mut self, geo_ids: &Vec<i64>) -> Self {
        self.geo_id = geo_ids.clone();
        self
    }
    fn check_phrase(phrase: &str) -> Result<&str, WordstatError> {
        if phrase.contains("&") {
            return Err(WordstatError::BadKeyphrase { reason: "Cant use '&' in keyphrases" })
        }
        if phrase.contains("%") {
            return Err(WordstatError::BadKeyphrase { reason: "Cant use '%' in keyphrases" })
        }
        if phrase.contains("+") {
            return Err(WordstatError::BadKeyphrase { reason: "Cant use '+' in keyphrases" })
        }
        if phrase.contains(" - ") {
            return Err(WordstatError::BadKeyphrase { reason: "Cant use ' - ' in keyphrases" })
        }
        if phrase.contains(":") {
            return Err(WordstatError::BadKeyphrase { reason: "Cant use ':' in keyphrases" })
        }
        Ok(phrase)
    }
}

/// Sends the request to the API using Wordstat client to start the report generation.
pub async fn create_report(client: &Client, request: &ReportRequest) -> Result<i64, WordstatError> {
    let method = "CreateNewWordstatReport";
    let mut params = serde_json::Map::new();
    params.insert("Phrases".to_string(), Value::from(request.phrases.clone()));
    params.insert("GeoID".to_string(), Value::from(request.geo_id.clone()));
    let result = client.post(method, Some(params.into())).await?;

    check_status(&result)?;

    let Some(data) = result.get("data") else { return Err(WordstatError::BadResponse{ reason: "Data field not found in response" }) };
    let Value::Number(report_id) = data else { return Err(WordstatError::BadResponse{ reason: "Data field is not a number" }) };
    if !report_id.is_i64() { return Err(WordstatError::BadResponse{ reason: "Data field is not an integer" }) }

    Ok(report_id.as_i64().unwrap() as i64)
}
