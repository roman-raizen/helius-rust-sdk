use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::types::Attribute;

pub fn deserialize_optional_attributes<'de, D>(deserializer: D) -> Result<Option<Vec<Attribute>>, D::Error>
where
    D: Deserializer<'de>,
{
    let single_or_vec: Option<Value> = Option::deserialize(deserializer)?;
    match single_or_vec {
        Some(Value::Array(vec)) => {
            let attributes: Vec<Attribute> = serde_json::from_value(Value::Array(vec))
                .map_err(serde::de::Error::custom)?;
            Ok(Some(attributes))
        }
        Some(Value::Object(obj)) => {
            let attribute: Attribute = serde_json::from_value(Value::Object(obj))
                .map_err(serde::de::Error::custom)?;
            Ok(Some(vec![attribute]))
        }
        _ => Ok(None), 
    }
}
