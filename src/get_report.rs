use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;

#[derive(Debug)]
pub struct WordstatItem {
    pub phrase: String,
    pub shows: i64
}

#[derive(Debug)]
pub struct Report {
    pub phrase: String,
    pub geo_id: Vec<i64>,
    pub searched_with: Vec<WordstatItem>,
    pub searched_also: Vec<WordstatItem>
}

pub async fn get_reports(client: &Client, report_id: i64) -> Result<Vec<Report>, WordstatError> {
    let method = "GetWordstatReport";
    let params = Value::Number(report_id.into());
    let result = client.post(method, Some(params)).await?;

    check_status(&result)?;

    let Some(data_val) = result.get("data") else { return Err(WordstatError::BadResponse) };
    let Value::Array(data) = data_val else { return Err(WordstatError::BadResponse) };

    parse_reports(&data)
}

fn parse_reports(data: &Vec<Value>) -> Result<Vec<Report>, WordstatError> {
    let mut reports: Vec<Report> = vec![];

    for item in data {
        reports.push(parse_report(item)?);
    }

    Ok(reports)
}

fn parse_report(data: &Value) -> Result<Report, WordstatError> {
    let Some(phrase_val) = data.get("Phrase") else { return Err(WordstatError::BadResponse) };
    let Value::String(phrase) = phrase_val else { return Err(WordstatError::BadResponse) };

    let Some(geoid_val) = data.get("GeoID") else { return Err(WordstatError::BadResponse) };
    let Value::Array(geoid_arr) = geoid_val else { return Err(WordstatError::BadResponse) };
    let geo_id = parse_geoid(geoid_arr)?;

    let Some(searched_with_val) = data.get("SearchedWith") else { return Err(WordstatError::BadResponse) };
    let Value::Array(searched_with_arr) = searched_with_val else { return Err(WordstatError::BadResponse) };
    let searched_with = parse_wordstat_items(searched_with_arr)?;

    let Some(searched_also_val) = data.get("SearchedWith") else { return Err(WordstatError::BadResponse) };
    let Value::Array(searched_also_arr) = searched_also_val else { return Err(WordstatError::BadResponse) };
    let searched_also = parse_wordstat_items(searched_also_arr)?;

    Ok(Report {
        phrase: phrase.to_string(),
        geo_id,
        searched_with,
        searched_also
    })
}

fn parse_geoid(data: &Vec<Value>) -> Result<Vec<i64>, WordstatError> {
    let mut geoids: Vec<i64> = vec![];

    for item in data {
        let Value::Number(geoid) = item else { return Err(WordstatError::BadResponse) };
        if !geoid.is_i64() { return Err(WordstatError::BadResponse) }
        geoids.push(geoid.as_i64().unwrap())
    }

    Ok(geoids)
}

fn parse_wordstat_items(data: &Vec<Value>) -> Result<Vec<WordstatItem>, WordstatError> {
    let mut items: Vec<WordstatItem> = vec![];

    for item in data {
        items.push(parse_wordstat_item(item)?);
    }

    Ok(items)
}

fn parse_wordstat_item(data: &Value) -> Result<WordstatItem, WordstatError> {
    let Some(phrase_val) = data.get("Phrase") else { return Err(WordstatError::BadResponse) };
    let Value::String(phrase) = phrase_val else { return Err(WordstatError::BadResponse) };

    let Some(shows_val) = data.get("Shows") else { return Err(WordstatError::BadResponse) };
    let Some(shows) = shows_val.as_i64() else { return Err(WordstatError::BadResponse) };

    Ok(WordstatItem {
        phrase: phrase.clone(),
        shows
    })
}

// TODO write unit tests
