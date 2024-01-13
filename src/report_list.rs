use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;


#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Done,
    Pending,
    Failed,
    Unknown
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ReportStatus {
    pub report_id: i64,
    pub status: StatusCode
}

pub async fn get_report_list(client: &Client) -> Result<Vec<ReportStatus>, WordstatError> {
    let method = "GetWordstatReportList";
    let result = client.post(method, None).await?;

    check_status(&result)?;

    let Some(data) = result.get("data") else { return Err(WordstatError::BadResponse{ reason: "No data field in response" }) };
    let Value::Array(reports) = data else { return Err(WordstatError::BadResponse{ reason: "Data field is not an array" }) };

    parse_reports(&reports)
}

fn parse_reports(data: &Vec<Value>) -> Result<Vec<ReportStatus>, WordstatError> {
    let mut reports: Vec<ReportStatus> = vec![];

    for report in data {
        reports.push(parse_report(report)?);
    }

    Ok(reports)
}

fn parse_report(report: &Value) -> Result<ReportStatus, WordstatError> {
    let Some(id_val) = report.get("ReportID") else { return Err(WordstatError::BadResponse{ reason: "No ReportID field" }) };
    let Some(report_id) = id_val.as_i64() else { return Err(WordstatError::BadResponse{ reason: "ReportID field is not an integer" }) };

    let Some(status_val) = report.get("StatusReport") else { return Err(WordstatError::BadResponse{ reason: "No StatusReport field" }) };
    let Value::String(status_str) = status_val else { return Err(WordstatError::BadResponse{ reason: "StatusReport field is not a string" }) };
    let status = match status_str.as_str() {
        "Done"      => { StatusCode::Done }
        "Pending"   => { StatusCode::Pending }
        "Failed"    => { StatusCode::Failed }
        _           => { StatusCode::Unknown }
    };

    Ok(ReportStatus {
        report_id,
        status
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_report() {
        let data = r#"
                {"ReportID":54312,"StatusReport":"Done"}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = super::parse_report(&input).unwrap();


        let expected = ReportStatus {
            report_id: 54312,
            status: StatusCode::Done
        };
        assert_eq!(received, expected)
    }

    #[test]
    #[should_panic]
    fn parse_invalid_report() {
        let data = r#"
                {"Report":54312,"StatusReport":"Done"}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = super::parse_report(&input).unwrap();


        let expected = ReportStatus {
            report_id: 54312,
            status: StatusCode::Done
        };
        assert_eq!(received, expected)
    }

    #[test]
    fn get_reposts_list() {
        let data = r#"
                {"data" : 
                [
                    {"ReportID":54312,"StatusReport":"Done"},
                    {"ReportID":542,"StatusReport":"Pending"},
                    {"ReportID":5423,"StatusReport":"Failed"},
                    {"ReportID":5424,"StatusReport":"what"}
                ]}
            "#;
        let return_value = serde_json::from_str(data).unwrap();

        let mut mock_client = Client::default();
        mock_client.expect_post()
            .withf(|method, _params| method == "GetWordstatReportList")
            .return_once(move |_method, _params| Ok(return_value));


        let received = futures::executor::block_on(super::get_report_list(&mock_client)).unwrap();


        let expected = vec![
            ReportStatus {
                report_id: 54312,
                status: StatusCode::Done,
            },
            ReportStatus {
                report_id: 542,
                status: StatusCode::Pending,
            },
            ReportStatus {
                report_id: 5423,
                status: StatusCode::Failed,
            },
            ReportStatus {
                report_id: 5424,
                status: StatusCode::Unknown,
            },
        ];

        assert_eq!(received, expected)
    }
}
