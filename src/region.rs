use serde_json::Value;
use crate::{WordstatError, check_status};
use mockall_double::double;
#[double] // For mocking the client in unit tests
use crate::client::Client;


#[derive(Debug, PartialEq, Eq)]
pub struct Region {
    pub name: String,
    pub id: i64,
    pub parent_id: Option<i64>,
}

pub async fn get_regions(client: &Client) -> Result<Vec<Region>, WordstatError> {
    let method = "GetRegions";
    let result = client.post(method, None).await?;

    check_status(&result)?;

    let Some(data) = result.get("data") else { return Err(WordstatError::BadResponse{ reason: "No data field in response" }) };
    let Value::Array(regions) = data else { return Err(WordstatError::BadResponse{ reason: "Data field does not contain an array" }) };

    parse_data(&regions)
}

fn parse_data(data: &Vec<Value>) -> Result<Vec<Region>, WordstatError> {
    let mut regions: Vec<Region> = vec![];

    for reg in data {
        regions.push(parse_region(reg)?);
    }

    Ok(regions)
}

fn parse_region(reg: &Value) -> Result<Region, WordstatError> {
    let Some(name_val) = reg.get("RegionName") else { return Err(WordstatError::BadResponse{ reason: "No RegionName field" }) };
    let Value::String(name) = name_val else { return Err(WordstatError::BadResponse{ reason: "RegionName field is not a string" }) };
    let Some(parent_id_val) = reg.get("ParentID") else { return Err(WordstatError::BadResponse{ reason: "No ParentID field" }) };
    let parent_id: Option<i64> = match parent_id_val {
        Value::Null => { None }
        Value::Number(num) => { Some(num.as_i64().unwrap()) } // Unsafe but unlikely to panic
        _ => { return Err(WordstatError::BadResponse{ reason: "ParentID field is not null and not a number" }); }
    };
    let Some(id_val) = reg.get("RegionID") else { return Err(WordstatError::BadResponse{ reason: "No RegionID field" }) };
    let Some(id) = id_val.as_i64() else { return Err(WordstatError::BadResponse{ reason: "RegionID is not an integer" }) };

    Ok(Region {
        name: name.clone(),
        id, parent_id
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_region_europe() {
        let data = r#"
                {"RegionName":"Europe","RegionID":111,"ParentID":0}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = parse_region(&input).unwrap();


        let expected = Region {
            name: "Europe".to_string(),
            id: 111,
            parent_id: Some(0)
        };
        assert_eq!(received, expected)
    }

    #[test]
    fn parse_region_all() {
        let data = r#"
                {"ParentID":null,"RegionName":"All","RegionID":0}
            "#;
        let input: Value = serde_json::from_str(data).unwrap();


        let received = parse_region(&input).unwrap();


        let expected = Region {
            name: "All".to_string(),
            id: 0,
            parent_id: None
        };
        assert_eq!(received, expected)
    }

    #[test]
    fn parse_regions() {
        let data = r#"
                    [{"ParentID":null,"RegionName":"All","RegionID":0},
                     {"RegionName":"Europe","RegionID":111,"ParentID":0},
                     {"RegionID":138,"RegionName":"Australia and Oceania","ParentID":0},
                     {"ParentID":0,"RegionID":166,"RegionName":"CIS (except Russia)"},
                     {"ParentID":0,"RegionID":183,"RegionName":"Asia"},
                     {"RegionName":"Russia","RegionID":225,"ParentID":0},
                     {"RegionName":"Africa","RegionID":241,"ParentID":0},
                     {"ParentID":0,"RegionID":977,"RegionName":"Republic of Crimea"},
                     {"RegionName":"North America","RegionID":10002,"ParentID":0}
                    ]
            "#;
        let input: Value = serde_json::from_str(data).unwrap();
        let Value::Array(input_vec) = input else { panic!("Something wrong with serde") };


        let received = parse_data(&input_vec).unwrap();


        let expected = vec![
            Region { name: "All".to_string(), id: 0, parent_id: None },
            Region { name: "Europe".to_string(), id: 111, parent_id: Some(0) }, 
            Region { name: "Australia and Oceania".to_string(), id: 138, parent_id: Some(0) }, 
            Region { name: "CIS (except Russia)".to_string(), id: 166, parent_id: Some(0) }, 
            Region { name: "Asia".to_string(), id: 183, parent_id: Some(0) }, 
            Region { name: "Russia".to_string(), id: 225, parent_id: Some(0) }, 
            Region { name: "Africa".to_string(), id: 241, parent_id: Some(0) }, 
            Region { name: "Republic of Crimea".to_string(), id: 977, parent_id: Some(0) }, 
            Region { name: "North America".to_string(), id: 10002, parent_id: Some(0) }];
        assert_eq!(received, expected)
    }


    #[test]
    fn get_regions() {
        let data = r#"
                    {"data":
                        [{"ParentID":null,"RegionName":"All","RegionID":0},
                         {"RegionName":"Europe","RegionID":111,"ParentID":0},
                         {"RegionID":138,"RegionName":"Australia and Oceania","ParentID":0},
                         {"ParentID":0,"RegionID":166,"RegionName":"CIS (except Russia)"},
                         {"ParentID":0,"RegionID":183,"RegionName":"Asia"},
                         {"RegionName":"Russia","RegionID":225,"ParentID":0},
                         {"RegionName":"Africa","RegionID":241,"ParentID":0},
                         {"ParentID":0,"RegionID":977,"RegionName":"Republic of Crimea"},
                         {"RegionName":"North America","RegionID":10002,"ParentID":0}
                        ]
                    }
            "#;
        let return_value = serde_json::from_str(data).unwrap();

        let mut mock_client = Client::default();
        mock_client.expect_post()
            .withf(|method, _params| method == "GetRegions")
            .return_once(move |_method, _params| Ok(return_value));


        let received = futures::executor::block_on(super::get_regions(&mock_client)).unwrap();


        let expected = vec![
            Region { name: "All".to_string(), id: 0, parent_id: None },
            Region { name: "Europe".to_string(), id: 111, parent_id: Some(0) }, 
            Region { name: "Australia and Oceania".to_string(), id: 138, parent_id: Some(0) }, 
            Region { name: "CIS (except Russia)".to_string(), id: 166, parent_id: Some(0) }, 
            Region { name: "Asia".to_string(), id: 183, parent_id: Some(0) }, 
            Region { name: "Russia".to_string(), id: 225, parent_id: Some(0) }, 
            Region { name: "Africa".to_string(), id: 241, parent_id: Some(0) }, 
            Region { name: "Republic of Crimea".to_string(), id: 977, parent_id: Some(0) }, 
            Region { name: "North America".to_string(), id: 10002, parent_id: Some(0) }];
        assert_eq!(received, expected)
    }
}
