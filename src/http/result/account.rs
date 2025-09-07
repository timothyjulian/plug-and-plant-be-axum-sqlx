#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResult {
    pub saved_account: SavedAccount,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SavedAccount {
    pub email: String,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub logged_account: LoggedAccount,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggedAccount {
    email: String,
    session_id: String,
    session_expire_time: String,
}
