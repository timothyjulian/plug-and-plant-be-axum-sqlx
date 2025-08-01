use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

use crate::config::Config;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub path: String,
    pub method: String,
    pub metadata: HashMap<String, String>,
}

impl RequestContext {
    pub fn new(method: String, path: String, request_id: String) -> Self {
        Self {
            request_id,
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

#[derive(Clone, Debug)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: PgPool,
}
