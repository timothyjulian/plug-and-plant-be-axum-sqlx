use serde_json::Value;

use crate::http::result::app_result::HttpError;

pub trait ValidateFieldsJSON {
    fn validate_required_fields(payload: &Value) -> Result<(), String> {
        let required_fields = Self::get_mandatory_field();
        let mut missing_fields = Vec::new();

        if let Value::Object(map) = payload {
            for field in required_fields {
                if !map.contains_key(field) {
                    missing_fields.push(field);
                } else if let Some(value) = map.get(field) {
                    // Check if the field is null or empty string
                    match value {
                        Value::Null => missing_fields.push(field),
                        Value::String(s) if s.is_empty() => missing_fields.push(field),
                        _ => {}
                    }
                }
            }
        } else {
            return Err("Payload must be a JSON object".to_string());
        }

        if !missing_fields.is_empty() {
            let error_msg = format!("Invalid Mandatory Field {}", missing_fields[0]);
            return Err(error_msg);
        }

        Ok(())
    }

    fn get_mandatory_field() -> Vec<&'static str>;

    fn validate_business_logic(&self) -> Result<(), HttpError>;
}
