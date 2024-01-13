use serde_json::Value;
use crate::{WordstatError, check_status};
use crate::client::Client;

pub async fn delete_report(client: &Client, report_id: i64) -> Result<(), WordstatError> {
    let method = "DeleteWordstatReport";
    let params = Value::Number(report_id.into());
    let result = client.post(method, Some(params)).await?;

    check_status(&result)?;
    let Some(data_val) = result.get("data") else { return Err(WordstatError::BadResponse{ reason: "Data field not found in response" }) };
    let Some(return_code) = data_val.as_i64() else { return Err(WordstatError::BadResponse{ reason: "Data field is not an integer" }) };

    if return_code != 1 {
        Err(WordstatError::UnknownError)
    }
    else {
        Ok(())
    }
}
