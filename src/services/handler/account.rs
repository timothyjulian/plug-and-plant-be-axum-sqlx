use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    dal::account::{fetch_account_by_email, register_account},
    services::utils::error::AppError,
};

pub struct AccountService;

impl AccountService {
    pub async fn register(pool: &PgPool, email: &str, password: &str) -> Result<(), AppError> {
        let account =
            fetch_account_by_email(pool, email)
                .await
                .map_err(|err| AppError::SqlxError {
                    msg: format!("Failed to query: {}", err),
                })?;

        if let Some(account) = account {
            return Err(AppError::EmailRegistered { account });
        }

        let password = &hash_password(password);

        if let Err(err) = register_account(pool, email, password).await {
            return Err(AppError::SqlxError {
                msg: format!("Failed to insert: {}", err),
            });
        }
        Ok(())
    }
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
