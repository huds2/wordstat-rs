pub mod region;
pub mod client;
pub mod create_report;
pub mod report_list;
pub mod get_report;
pub mod delete_report;

pub use client::Client;
pub use create_report::{ReportRequest, create_report};
pub use delete_report::delete_report;
pub use get_report::{ReportEntry, WordstatItem, get_report};
pub use region::{Region, get_regions};
pub use report_list::{ReportStatus, StatusCode, get_report_list};

use custom_error::custom_error;
use serde_json::Value;

custom_error!{pub WordstatError
    BadResponse                                     = "Response had bad structure",
    UnknownResponseCode{code:i64}                   = "Unknown response code recieved: {code}",
    UnknownError                                    = "Unknown error has occured",
    ReportDoesNotExist                              = "The specified report does not exist",        // code 24, 91
    InvalidReportId                                 = "The specified report ID is not valid",       // code 22, 93
    ReportQueueFull                                 = "The report queue if full",                   // code 31
    QuotaExhausted                                  = "The report quota has been exhausted",        // code 152
    AuthorizationError                              = "Invalid login, token or token has expired",  // code 53
    AccessDenied                                    = "Access to Yandex Direct API has been denied",// code 58
    InternalServerError                             = "Internal server error",                      // code 500
    InvalidRequest                                  = "The request was invalid",                    // code 501
    ReportNotReady                                  = "The report is not ready yet"                 // code 74, 92
}

fn check_status(response: &Value) -> Result<(), WordstatError> {
    let Some(error_code_val) = response.get("error_code") else { return Ok(()) };
    let Some(error_code) = error_code_val.as_i64() else { return Err(WordstatError::BadResponse) };

    match error_code {
        24 | 91     => { Err(WordstatError::ReportDoesNotExist) }
        22 | 93     => { Err(WordstatError::InvalidReportId) }
        31          => { Err(WordstatError::ReportQueueFull) }
        152         => { Err(WordstatError::QuotaExhausted) }
        53          => { Err(WordstatError::AuthorizationError) }
        58          => { Err(WordstatError::AccessDenied) }
        500         => { Err(WordstatError::InternalServerError) }
        501         => { Err(WordstatError::InvalidRequest) }
        74 | 92     => { Err(WordstatError::ReportNotReady) }
        _           => { Err(WordstatError::UnknownResponseCode { code: error_code }) }
    }
}
