use serde::Deserialize;

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<Guild>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Guild {
    pub values: GuildValues,
    pub modules: GuildModules,
    pub ignored: GuildIgnores,
    pub boosts: GuildBoosts,
    pub levelroles: Vec<GuildLevelRoles>,
    pub announce: GuildAnnounce,
    pub logs: GuildLogs,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct GuildValues {
    pub reactionxp: i64,
    pub fishXP: i64,
    pub lootXP: i64,
    pub messagecooldown: i64,
    pub messagexp: i64,
    pub rollXP: i64,
    pub voicejoincooldown: i64,
    pub voicexp: i64,
    pub gamecooldown: i64,
    pub maximumdailyxp: i64,
    pub triviacooldown: i64,
    pub triviaxp: i64,
    pub maximumlevel: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildModules {
    pub reactionxp: bool,
    pub maximumlevel: bool,
    pub autonick: bool,
    pub games: bool,
    pub messagexp: bool,
    pub resetonleave: bool,
    pub voicexp: bool,
    pub enablecommandsinthreads: bool,
    pub autonickshowstring: bool,
    pub autonickuseprefix: bool,
    pub trivia: bool,
    pub leaderboard: bool,
    pub removereachedlevelroles: bool,
    pub singlerankrole: bool,
    pub ignoreafk: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildLogs {
    pub voicetime: Option<String>,
    pub levelup: Option<String>,
    pub exceptions: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildIgnores {
    pub roles: Vec<String>,
    pub channels: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildBoosts {
    pub roles: Vec<GuildBoostObject>,
    pub channels: Vec<GuildBoostObject>,
    pub categories: Option<Vec<GuildBoostObject>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildBoostObject {
    pub id: String,
    pub percentage: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildLevelRoles {
    pub id: String,
    pub level: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildAnnounce {
    pub current: bool,
    pub message: String,
    pub ping: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct GuildPremiumResponse {
    pub premium: bool,
    pub voteFree: bool,
}

impl Guild {
    pub async fn from_id(guild_id: u64) -> DbResult<Guild> {
        let response = crate::get_json::<GuildResponse>(format!("/guild/{}", guild_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get guild with id {}", guild_id).into())
        }
    }

    pub async fn delete(
        guild_id: &u64,
    ) -> DbResult<Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>> {
        crate::delete_json(format!("/guild/{}", guild_id)).await?;

        Ok(Ok(()))
    }

    pub async fn delete_xp(
        guild_id: &u64,
    ) -> DbResult<Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>>> {
        crate::delete_json(format!("/guild/{}/members/xp", guild_id)).await?;

        Ok(Ok(()))
    }

    pub async fn is_premium(guild_id: &u64) -> DbResult<bool> {
        let response =
            crate::get_json::<GuildPremiumResponse>(format!("/guild/{}/premium", guild_id)).await?;

        Ok(response.premium)
    }
}
