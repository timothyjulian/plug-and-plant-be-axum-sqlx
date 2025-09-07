use crate::dal::account::Account;

#[derive(Debug)]
pub enum AppError {
    EmailRegistered { account: Account },
    SqlxError { msg: String },
    InvalidCredentials { msg: String },
}
