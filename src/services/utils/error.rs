use crate::dal::account::Account;

pub enum AppError {
    EmailRegistered { account: Account },
    SqlxError { msg: String },
}
