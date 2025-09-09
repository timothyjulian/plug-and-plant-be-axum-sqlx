use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: String,
    pub account_id: i32,
    pub expiry_time: DateTime<Utc>,
    pub utc_create: DateTime<Utc>,
    pub utc_modified: DateTime<Utc>,
}

