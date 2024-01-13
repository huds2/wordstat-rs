//! # wordstat-rs
//! This crate makes it easy to interact with Yandex Direct API for getting
//! statistics about keyword searches from Wordstat service.
//!
//! ## Example
//! To start using the library you should first create the client the following way:
//! ```rust
//! # use wordstat_rs::*;
//! let client = Client::new("token", "api_url");
//! ```
//!
//! To get the list of available regions do the following:
//! ```rust,ignore
//! let regions = get_regions(&client).await.unwrap();
//! ```
//! This returns a [Result enum](Result), containing a [vector](Vec) of [regions](crate::region::Region)
//!
//! To start generating a report you should craete a
//! [ReportRequest](crate::create_report::ReportRequest) and then call
//! [create_report](crate::create_report::create_report) function.
//! ```rust,ignore
//! let request = ReportRequest::new()
//!     .add_phrase("rust lang").unwrap()
//!     .add_geo(101);
//! let report_id = create_report(&client, &request).await.unwrap();
//! ```
//! 
//! To check the list of available reports and their statuses you can use:
//! ```rust,ignore
//! let report_list = get_report_list(&client).await;
//! ```
//! This returns a [Result enum](Result), containing a [vector](Vec) of
//! [report statuses](crate::report_list::ReportStatus)
//!
//! If the report is ready, you can get it using:
//! ```rust,ignore
//! let report = get_reports(&client, report_id).await;
//! ```
//! This returns a [Result enum](Result), containing a [vector](Vec) of
//! [report entries](crate::get_report::ReportEntry) (one per keyphrase in the ReportRequest).
//!
//! To delete a report you should use:
//! ```rust,ignore
//! delete_report(&client, 11053065).await.unwrap();
//! ```
//!
//! ## Usage notes
//!
//! While using the library keep in mind:
//! - One ReportRequest can contain up to 10 keyphrases
//! - The server stores up to five reports simultaneously, so you should delete the report once you
//! have downloaded its data
//! - Geo is optional when creating a ReportRequest
//!
//! ## API URLs
//!
//! API version 4 should be used
//! The API URL is <https://api.direct.yandex.ru/v4/json/>.
//! If your token is for the API sandbox you should use <https://api-sandbox.direct.yandex.ru/v4/json/>
//! as the URL.
//!
//! ## Getting the API token
//!
//! 1. Create an application that will be using the Yandex Direct API
//! [here](https://oauth.yandex.ru/client/new)
//! 2. Recieve access to the API by filing the form 
//! [here](https://direct.yandex.ru/registered/main.pl?cmd=apiCertificationRequestList)
//! 3. Turn on the sandbox mode 
//! [here](https://direct.yandex.ru/registered/main.pl?cmd=apiApplicationList)
//! 4. Get the token by authorizing in your app by following this link:
//! <https://oauth.yandex.ru/authorize?response_type=token&client_id=[app_client_id]>
//! Don't forget to replace the ```app_client_id``` with the client_id of your app.

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
    BadResponse{reason: &'static str}               = "Response had bad structure",
    BadKeyphrase{reason: &'static str}              = "Bad keyphrase supplied",
    TooManyKeyphrases                               = "Too many keyphrases were supplied",
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
    ReportNotReady                                  = "The report is not ready yet",                // code 74, 92
    InvalidRequestParameters                        = "The reqeust parameters were invalid"         // code 71
}

fn check_status(response: &Value) -> Result<(), WordstatError> {
    let Some(error_code_val) = response.get("error_code") else { return Ok(()) };
    let Some(error_code) = error_code_val.as_i64() else { return Err(WordstatError::BadResponse{ reason: "Error code is not an integer" } ) };

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
        71          => { Err(WordstatError::InvalidRequestParameters) }
        _           => { Err(WordstatError::UnknownResponseCode { code: error_code }) }
    }
}
