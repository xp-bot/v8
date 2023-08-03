use log::error;
use reqwest::Error;

use crate::utils::api::get_authenticated;

use super::datatypes::{GuildMember, GuildMemberResponse};

pub async fn get_guild_member_by_id(guild_id: u64, member_id: u64) -> Result<GuildMember, Error> {
    let response: Result<GuildMemberResponse, Error> = get_authenticated(format!("/guild/{}/member/{}", guild_id, member_id)).await.json().await;

    match response {
        Ok(member) => Ok(member.content),
        Err(why) => {
            error!("Could not get guild member: {:?}", why);
            Err(why)
        }
    }
}