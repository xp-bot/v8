use serde::{Deserialize, Serialize};

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<GuildMember>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMember {
    pub xp: u64,
    pub userData: GuildMemberData,
    pub settings: GuildMemberSettings,
    pub timestamps: GuildMemberTimestamps,
    pub streaks: GuildMemberStreaks,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMemberData {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMemberSettings {
    pub incognito: Option<bool>,
    pub ranking: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMemberTimestamps {
    pub message_cooldown: Option<u64>,
    pub game_trivia: Option<u64>,
    pub game_daily: Option<u64>,
    pub game_fish: Option<u64>,
    pub game_loot: Option<u64>,
    pub game_roll: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMemberStreaks {
    pub game_daily: Option<u64>,
    pub game_trivia: Option<u64>,
    pub daily: Option<u64>, // deprecated
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct XPPostBody {
    xp: u64,
    userData: GuildMemberData,
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

    pub async fn set_xp(guild_id: u64, member_id: u64, xp: &u64, guild_member: &GuildMember) -> DbResult<Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>> {
        let response = crate::post_json(format!("/guild/{}/member/{}/direct/xp", guild_id, member_id), XPPostBody {
            xp: *xp,
            userData: guild_member.userData.clone(),
        }).await;

        match response {
            Ok(_) => Ok(Ok(())),
            Err(e) => Ok(Err(e.into())),
        }
    }

    pub async fn set_guild_member(guild_id: u64, member_id: u64, guild_member: GuildMember) -> DbResult<Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>> {
        let response = crate::patch_json(format!("/guild/{}/member/{}", guild_id, member_id), guild_member).await;

        match response {
            Ok(_) => Ok(Ok(())),
            Err(e) => Ok(Err(e.into())),
        }
    }
}
