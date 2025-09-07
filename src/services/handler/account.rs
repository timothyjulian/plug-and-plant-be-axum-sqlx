use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    dal::account::{fetch_account_by_email, insert_account},
    services::utils::error::AppError,
};

pub async fn register_user(pool: &PgPool, email: &str, password: &str) -> Result<(), AppError> {
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
    Ok(())
}

pub async fn login(pool: &PgPool, email: &str, password: &str) -> Result<(), AppError> {
    Ok(())
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
