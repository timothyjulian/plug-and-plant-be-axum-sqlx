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
