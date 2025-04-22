use crate::error::{WebullError, WebullResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Convert a struct to a JSON string.
pub fn to_json<T>(value: &T) -> WebullResult<String>
where
    T: Serialize,
{
    serde_json::to_string(value).map_err(|e| WebullError::SerializationError(e))
}

/// Convert a JSON string to a struct.
pub fn from_json<T>(json: &str) -> WebullResult<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(json).map_err(|e| WebullError::SerializationError(e))
}

/// Convert a struct to a JSON value.
pub fn to_json_value<T>(value: &T) -> WebullResult<Value>
where
    T: Serialize,
{
    serde_json::to_value(value).map_err(|e| WebullError::SerializationError(e))
}

/// Convert a JSON value to a struct.
pub fn from_json_value<T>(value: Value) -> WebullResult<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(value).map_err(|e| WebullError::SerializationError(e))
}

/// Build a JSON object with the given parameters.
pub fn build_json_object(params: &[(&str, Value)]) -> Value {
    let mut obj = json!({});

    for (key, value) in params {
        obj[key] = value.clone();
    }

    obj
}
