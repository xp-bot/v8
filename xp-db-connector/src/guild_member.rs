use serde::Deserialize;

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<GuildMember>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMember {
    pub xp: usize,
    pub userData: GuildMemberData,
    pub settings: GuildMemberSettings,
    pub timestamps: GuildMemberTimestamps,
    pub streaks: GuildMemberStreaks,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberData {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberSettings {
    pub incognito: Option<bool>,
    pub ranking: Option<bool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberTimestamps {
    pub message_cooldown: Option<u64>,
    pub game_trivia: Option<u64>,
    pub game_daily: Option<u64>,
    pub game_fish: Option<u64>,
    pub game_loot: Option<u64>,
    pub game_roll: Option<u64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberStreaks {
    pub game_daily: Option<u64>,
    pub game_trivia: Option<u64>,
    pub daily: Option<u64>, // deprecated
}


impl GuildMember {
    pub async fn from_id(guild_id: u64, member_id: u64) -> DbResult<GuildMember> {
        let response = crate::get_json::<GuildMemberResponse>(format!("/guild/{}/member/{}", guild_id, member_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get guild member: {}", response.message).into())
        }
    }
}
