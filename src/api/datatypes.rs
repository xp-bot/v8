use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberResponse {
    pub success: bool,
    pub message: String,
    pub content: GuildMember,
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
    pub incognito: bool,
    pub ranking: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberTimestamps {
    pub message_cooldown: Option<u64>,
    pub join_voicechat: Option<u64>,
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