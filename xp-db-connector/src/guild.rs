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
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct GuildValues {
    pub reactionxp: u32,
    pub fishXP: u32,
    pub lootXP: u32,
    pub messagecooldown: u32,
    pub messagexp: u32,
    pub rollXP: u32,
    pub voicejoincooldown: u32,
    pub voicexp: u32,
    pub gamecooldown: u32,
    pub maximumdailyxp: u32,
    pub triviacooldown: u32,
    pub triviaxp: u32,
    pub maximumlevel: u32,
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
    pub percentage: u32,
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

impl Guild {
    pub async fn from_id(guild_id: u64) -> DbResult<Guild> {
        let response = crate::get_json::<GuildResponse>(format!("/guild/{}", guild_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get guild with id {}", guild_id).into())
        }
    }
}
