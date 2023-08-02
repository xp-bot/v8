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
    pub incognito: Option<bool>,
    pub ranking: Option<bool>,
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

#[derive(Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub content: User,
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

#[derive(Deserialize, Clone, Debug)]
pub struct UserBackgroundResponse {
    pub success: bool,
    pub message: String,
    pub content: UserBackground,
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