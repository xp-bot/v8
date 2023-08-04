use serde::Deserialize;

use crate::DbResult;

#[derive(Deserialize, Clone, Debug)]
pub struct GuildPremiumResponse {
    pub success: bool,
    pub message: String,
    pub content: Option<GuildPremium>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Debug)]
pub struct GuildPremium {
    pub premium: bool,
    pub voteFree: bool,
}

impl GuildPremium {
    pub async fn from_id(guild_id: u64) -> DbResult<GuildPremium> {
        let response =
            crate::get_json::<GuildPremiumResponse>(format!("/guild/{}/premium", guild_id)).await?;

        if response.success && response.content.is_some() {
            Ok(response.content.unwrap())
        } else {
            Err(format!("Failed to get guild premium with id {}", guild_id).into())
        }
    }
}
