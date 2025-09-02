use anyhow::{Error, Ok};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

#[derive(FromRow, Debug)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub utc_create: DateTime<Utc>,
    pub utc_modified: DateTime<Utc>,
}

pub async fn fetch_account_by_email(
    pool: &PgPool,
    email: &String,
) -> Result<Option<Account>, Error> {
    let account: Option<Account> = sqlx::query_as("SELECT * FROM account WHERE email = $1;")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(account)
}

pub async fn register_account(
    pool: &PgPool,
    email: &String,
    password: &String,
) -> Result<(), Error> {
    let now = Utc::now().naive_utc(); // This is UTC time
    sqlx::query(
        "INSERT INTO account(email, password, utc_create, utc_modified) VALUES ($1, $2, $3, $4);",
    )
    .bind(email)
    .bind(password)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
