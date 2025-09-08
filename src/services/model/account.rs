#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SavedAccount {
    pub email: String,
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggedAccount {
    pub email: String,
    pub session_id: String,
    pub session_expire_time: String,
}
