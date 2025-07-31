use anyhow::{Error, Ok};
use chrono::{DateTime, Local, NaiveDateTime};
use sqlx::{prelude::FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub utc_create: DateTime<Local>,
    pub utc_modified: DateTime<Local>,
}

pub async fn fetch_account_by_email(pool: &PgPool, email: String) -> Result<Option<Account>, Error> {
    let account: Option<Account> = sqlx::query_as("SELECT * FROM account WHERE email = $1;").bind(email).fetch_optional(pool).await?;
    Ok(account)
}

pub async fn register_account(pool: &PgPool, email: String, password: String) -> Result<(), Error> {
    sqlx::query("INSERT INTO account(email, password, utc_create, utc_modified) VALUES ($1, $2, $3, $4);").bind(email).bind(password).bind(Local::now().naive_local()).bind(Local::now().naive_local()).execute(pool).await?;
    Ok(())
}