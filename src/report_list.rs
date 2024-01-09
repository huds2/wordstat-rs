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
pub struct ReportStatus {
    pub report_id: i64,
    pub status: StatusCode
}

pub async fn get_report_list(client: &Client) -> Result<Vec<ReportStatus>, WordstatError> {
    let method = "GetWordstatReportList";
    let result = client.post(method, None).await?;

    check_status(&result)?;

    let Some(data) = result.get("data") else { return Err(WordstatError::BadResponse) };
    let Value::Array(reports) = data else { return Err(WordstatError::BadResponse) };

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
    let Some(id_val) = report.get("ReportID") else { return Err(WordstatError::BadResponse) };
    let Some(report_id) = id_val.as_i64() else { return Err(WordstatError::BadResponse) };

    let Some(status_val) = report.get("StatusReport") else { return Err(WordstatError::BadResponse) };
    let Value::String(status_str) = status_val else { return Err(WordstatError::BadResponse) };
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

// TODO write unit tests
