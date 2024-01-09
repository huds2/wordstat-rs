use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WordstatItem {
    pub phrase: String,
    pub shows: i64
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ReportEntry {
    pub phrase: String,
    pub geo_id: Vec<i64>,
    pub searched_with: Vec<WordstatItem>,
    pub searched_also: Vec<WordstatItem>
}

pub async fn get_report(client: &Client, report_id: i64) -> Result<Vec<ReportEntry>, WordstatError> {
    let method = "GetWordstatReport";
    let params = Value::Number(report_id.into());
    let result = client.post(method, Some(params)).await?;

    check_status(&result)?;


    let Some(data_val) = result.get("data") else { return Err(WordstatError::BadResponse) };
    let Value::Array(data) = data_val else { return Err(WordstatError::BadResponse) };

    parse_report(&data)
}

fn parse_report(data: &Vec<Value>) -> Result<Vec<ReportEntry>, WordstatError> {
    let mut report: Vec<ReportEntry> = vec![];

    for item in data {
        report.push(parse_report_entry(item)?);
    }

    Ok(report)
}

fn parse_report_entry(data: &Value) -> Result<ReportEntry, WordstatError> {
    let Some(phrase_val) = data.get("Phrase") else { return Err(WordstatError::BadResponse) };
    let Value::String(phrase) = phrase_val else { return Err(WordstatError::BadResponse) };

    let Some(geoid_val) = data.get("GeoID") else { return Err(WordstatError::BadResponse) };
    let Value::Array(geoid_arr) = geoid_val else { return Err(WordstatError::BadResponse) };
    let geo_id = parse_geoid(geoid_arr)?;

    let Some(searched_with_val) = data.get("SearchedWith") else { return Err(WordstatError::BadResponse) };
    let Value::Array(searched_with_arr) = searched_with_val else { return Err(WordstatError::BadResponse) };
    let searched_with = parse_wordstat_items(searched_with_arr)?;

    let Some(searched_also_val) = data.get("SearchedAlso") else { return Err(WordstatError::BadResponse) };
    let Value::Array(searched_also_arr) = searched_also_val else { return Err(WordstatError::BadResponse) };
    let searched_also = parse_wordstat_items(searched_also_arr)?;

    Ok(ReportEntry {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wordstat_item() {
        let data = r#"
                {"Phrase":"rust-lang","Shows":528123}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = super::parse_wordstat_item(&input).unwrap();


        let expected = WordstatItem {
            phrase: "rust-lang".to_string(),
            shows: 528123
        };
        assert_eq!(received, expected)
    }

    #[test]
    #[should_panic]
    fn parse_invalid_wordstat_item() {
        let data = r#"
                {"Ph":"rust-lang","Shows":528123}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = super::parse_wordstat_item(&input).unwrap();


        let expected = WordstatItem {
            phrase: "rust-lang".to_string(),
            shows: 528123
        };
        assert_eq!(received, expected)
    }

    #[test]
    fn parse_report_entry() {
        let data = r#"
                {"Phrase": "rust-lang", "GeoID": [5, 4, 3],
                "SearchedWith":
                [ {"Phrase": "rust-lang", "Shows": 543}, {"Phrase": "rust-lang how", "Shows": 23}], 
                "SearchedAlso":
                [ {"Phrase": "cpp", "Shows": 432}, {"Phrase": "cpp worse than rust?", "Shows": 12}]}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = super::parse_report_entry(&input).unwrap();


        let expected = ReportEntry {
            phrase: "rust-lang".to_string(),
            geo_id: vec![5, 4, 3],
            searched_with: vec![
                WordstatItem {
                    phrase: "rust-lang".to_string(),
                    shows: 543
                },
                WordstatItem {
                    phrase: "rust-lang how".to_string(),
                    shows: 23
                }
            ],
            searched_also: vec![
                WordstatItem {
                    phrase: "cpp".to_string(),
                    shows: 432
                },
                WordstatItem {
                    phrase: "cpp worse than rust?".to_string(),
                    shows: 12
                }
            ],
        };
        assert_eq!(received, expected)
    }

    #[test]
    fn get_regions() {
        let data = r#"
                {"data" : 
                [
                    {"Phrase": "rust-lang", "GeoID": [5, 4, 3],
                    "SearchedWith":
                    [ {"Phrase": "rust-lang", "Shows": 543}, {"Phrase": "rust-lang how", "Shows": 23}], 
                    "SearchedAlso":
                    [ {"Phrase": "cpp", "Shows": 432}, {"Phrase": "cpp worse than rust?", "Shows": 12}]},

                    {"Phrase": "rust-langgg", "GeoID": [5, 4, 3],
                    "SearchedWith":
                    [ {"Phrase": "rust-lang", "Shows": 543}, {"Phrase": "rust-lang how", "Shows": 23}], 
                    "SearchedAlso":
                    [ {"Phrase": "cpp", "Shows": 432}, {"Phrase": "cpp worse than rust?", "Shows": 12}]}
                ]}
            "#;
        let return_value = serde_json::from_str(data).unwrap();

        let mut mock_client = Client::default();
        mock_client.expect_post()
            .withf(|method, _params| method == "GetWordstatReport")
            .return_once(move |_method, _params| Ok(return_value));


        let received = futures::executor::block_on(super::get_report(&mock_client, 54)).unwrap();


        let expected = vec![
            ReportEntry {
                phrase: "rust-lang".to_string(),
                geo_id: vec![5, 4, 3],
                searched_with: vec![
                    WordstatItem {
                        phrase: "rust-lang".to_string(),
                        shows: 543
                    },
                    WordstatItem {
                        phrase: "rust-lang how".to_string(),
                        shows: 23
                    }
                ],
                searched_also: vec![
                    WordstatItem {
                        phrase: "cpp".to_string(),
                        shows: 432
                    },
                    WordstatItem {
                        phrase: "cpp worse than rust?".to_string(),
                        shows: 12
                    }
                ],
            },
            ReportEntry {
                phrase: "rust-langgg".to_string(),
                geo_id: vec![5, 4, 3],
                searched_with: vec![
                    WordstatItem {
                        phrase: "rust-lang".to_string(),
                        shows: 543
                    },
                    WordstatItem {
                        phrase: "rust-lang how".to_string(),
                        shows: 23
                    }
                ],
                searched_also: vec![
                    WordstatItem {
                        phrase: "cpp".to_string(),
                        shows: 432
                    },
                    WordstatItem {
                        phrase: "cpp worse than rust?".to_string(),
                        shows: 12
                    }
                ],
            },
        ];

        assert_eq!(received, expected)
    }
}
