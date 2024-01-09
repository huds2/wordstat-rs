use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;

pub async fn delete_report(client: &Client, report_id: i64) -> Result<(), WordstatError> {
    let method = "DeleteWordstatReport";
    let params = Value::Number(report_id.into());
    let result = client.post(method, Some(params)).await?;

    check_status(&result)?;
    let Some(data_val) = result.get("data") else { return Err(WordstatError::BadResponse) };
    let Some(return_code) = data_val.as_i64() else { return Err(WordstatError::BadResponse) };

    if return_code != 1 {
        Err(WordstatError::UnknownError)
    }
    else {
        Ok(())
    }
}
