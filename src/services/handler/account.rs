use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    dal::account::{fetch_account_by_email, fetch_account_by_email_and_password, insert_account},
    services::{
        model::account::{LoggedAccount, SavedAccount},
        utils::error::AppError,
    },
};

// TODO change to account normal no service, Rusty way
pub async fn register_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<SavedAccount, AppError> {
    let account = fetch_account_by_email(pool, email)
        .await
        .map_err(|err| AppError::SqlxError {
            msg: format!("Failed to query: {}", err),
        })?;

    if let Some(account) = account {
        return Err(AppError::EmailRegistered { account });
    }

    let password = &hash_password(password);

    if let Err(err) = insert_account(pool, email, password).await {
        return Err(AppError::SqlxError {
            msg: format!("Failed to insert: {}", err),
        });
    }

    Ok(SavedAccount {
        email: email.to_string(),
    })
}

pub async fn login_user(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<LoggedAccount, AppError> {
    let password = hash_password(password);
    let account = fetch_account_by_email_and_password(pool, email, &password)
        .await
        .map_err(|err| AppError::SqlxError {
            msg: format!("Failed to query: {}", err),
        })?;

    if let None = account {
        return Err(AppError::InvalidCredentials {
            msg: String::from("Invalid Account"),
        });
    }

    Ok(LoggedAccount {
        email: account.unwrap().email,
        session_id: String::from("test"),
        session_expire_time: String::from("test"),
    })
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
