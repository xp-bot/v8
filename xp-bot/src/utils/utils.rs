use serenity::model::prelude::{ChannelId, RoleId};
use xp_db_connector::{guild::Guild, user::User};

use super::topgg;

pub fn calculate_total_boost_percentage(
    guild: Guild,
    role_ids: &Vec<RoleId>,
    channel_id: u64,
    category_id: Option<ChannelId>,
) -> f32 {
    log::info!("Role IDs: {:?}", role_ids);
    log::info!("Channel ID: {}", channel_id);
    if let Some(category_id) = category_id {
        log::info!("Category ID: {}", category_id);
    }

    let mut boost_percentage = 0.0;

    let role_boosts = guild.boosts.roles;
    let channel_boosts = guild.boosts.channels;
    let category_boosts = guild.boosts.categories;

    for boost in &role_boosts {
        if role_ids.contains(&boost.id.parse::<RoleId>().unwrap()) {
            boost_percentage += boost.percentage as f32 / 100 as f32;
        }
    }

    for boost in &channel_boosts {
        if boost.id.parse::<u64>().unwrap() == channel_id {
            boost_percentage += boost.percentage as f32 / 100 as f32;
        }
    }

    if let Some(category_id) = category_id {
        if category_boosts.is_none() {
            return boost_percentage;
        }

        for boost in &category_boosts.unwrap() {
            if boost.id.parse::<u64>().unwrap() == category_id.0 {
                boost_percentage += boost.percentage as f32 / 100 as f32;
            }
        }
    }

    boost_percentage
}

pub fn format_number(number: u64) -> String {
    let mut number = number.to_string();
    let mut formatted_number = String::new();

    while number.len() > 3 {
        formatted_number = format!(
            "{}{}",
            number.split_off(number.len() - 3),
            if formatted_number.is_empty() { "" } else { "," }
        );
    }

    format!("{}{}", number, formatted_number)
}

pub async fn eligibility_helper(user_id: u64) -> bool {
    let user = User::is_premium(user_id).await.unwrap_or(false);

    if user {
        return true;
    }

    let voted = topgg::check_user_vote(&user_id).await;

    if voted {
        return true;
    }

    false
}

pub fn is_cooldowned(timestamp_now: u64, timestamp_then: u64, cooldown: u64) -> bool {
    if timestamp_now - timestamp_then < cooldown {
        return true;
    }

    false
}
