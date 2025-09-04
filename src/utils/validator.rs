use serde_json::Value;

use crate::http::result::app_result::HttpError;

pub trait ValidateFieldsJSON {
    fn validate_required_fields(payload: &Value) -> Result<(), String>;

    fn validate_business_logic(&self) -> Result<(), HttpError>;
}
