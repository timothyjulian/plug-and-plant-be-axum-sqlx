use crate::services::model::account::{LoggedAccount, SavedAccount};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResult {
    pub saved_account: SavedAccount,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub logged_account: LoggedAccount,
}
