use serde::{Deserialize, Serialize};

use crate::DbResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<User>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub badges: Vec<String>,
    pub titles: Vec<String>,
    pub settings: UserSettings,
    pub timestamps: UserTimestamps,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTimestamps {
    pub message_cooldown: Option<u64>,
    pub join_voicechat: Option<u64>,
    pub game_trivia: Option<u64>,
    pub game_daily: Option<u64>,
    pub game_fish: Option<u64>,
    pub game_loot: Option<u64>,
    pub game_roll: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSettings {
    pub background: UserSettingsBackground,
    pub language: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSettingsBackground {
    pub bg: Option<i32>,
    pub blur: Option<i32>,
    pub custom: bool,
    pub canvas: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPremiumResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<UserPremium>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPremium {
    pub userPremium: bool,
    pub serverPremium: u32,
    pub servers: Vec<String>,
    pub voteFreeCount: u32,
    pub voteFreeServers: Vec<String>,
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

    pub async fn is_premium(user_id: u64) -> DbResult<bool> {
        let response =
            crate::get_json::<UserPremiumResponse>(format!("/user/{}/premium", user_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap().userPremium)
        } else {
            Err(format!("Failed to get user premium: {}", response.message).into())
        }
    }

    pub async fn set(
        user_id: u64,
        user: User,
    ) -> DbResult<Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>> {
        let response = crate::patch_json(format!("/user/{}", user_id), user).await;

        match response {
            Ok(_) => Ok(Ok(())),
            Err(e) => Ok(Err(e.into())),
        }
    }
}
