use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub path: String,
    pub method: String,
    pub metadata: HashMap<String, String>,
}

impl RequestContext {
    pub fn new(method: String, path: String) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string().replace("-", ""),
            path,
            method,
            metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl std::fmt::Display for RequestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {}]", self.request_id, self.method, self.path,)
    }
}
