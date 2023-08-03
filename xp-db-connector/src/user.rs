use serde::Deserialize;

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<User>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub badges: Vec<String>,
    pub titles: Vec<String>,
    pub settings: UserSettings,
    pub timestamps: UserTimestamps,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserTimestamps {
    pub message_cooldown: Option<u64>,
    pub join_voicechat: Option<u64>,
    pub game_trivia: Option<u64>,
    pub game_daily: Option<u64>,
    pub game_fish: Option<u64>,
    pub game_loot: Option<u64>,
    pub game_roll: Option<u64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserSettings {
    pub background: UserSettingsBackground,
    pub language: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserSettingsBackground {
    pub bg: Option<i32>,
    pub blur: Option<i32>,
    pub custom: bool,
    pub canvas: bool,
}

impl User {
    pub async fn from_id(user_id: u64) -> DbResult<User> {
        let response = crate::get_json::<UserResponse>(format!("/user/{}", user_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get user: {}", response.message).into())
        }
    }
}
