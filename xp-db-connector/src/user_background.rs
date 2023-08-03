use serde::Deserialize;

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct UserBackgroundResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<UserBackground>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserBackground {
    pub big: UserBackgroundBig,
    pub small: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserBackgroundBig {
    pub top: Option<String>,
    pub bottom: Option<String>,
}

impl UserBackground {
    pub async fn from_id(user_id: u64) -> DbResult<UserBackground> {
        let response = crate::get_json::<UserBackgroundResponse>(format!("/user/{}/background", user_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get user background: {}", response.message).into())
        }
    }
}
