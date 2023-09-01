use serenity::{model::prelude::{ChannelId, RoleId}, builder::CreateMessage};
use xp_db_connector::{guild::Guild, user::User};

use super::{topgg, colors};

pub fn calculate_total_boost_percentage(
    guild: Guild,
    role_ids: &Vec<RoleId>,
    channel_id: u64,
    category_id: Option<ChannelId>,
) -> f32 {
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

pub fn calculate_total_boost_percentage_by_ids(
    guild: Guild,
    role_ids: Vec<u64>,
    channel_id: u64,
    category_id: Option<u64>,
) -> f32 {
    let mut boost_percentage = 0.0;

    let role_boosts = guild.boosts.roles;
    let channel_boosts = guild.boosts.channels;
    let category_boosts = guild.boosts.categories;

    for boost in &role_boosts {
        if role_ids.contains(&boost.id.parse::<u64>().unwrap()) {
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
            if boost.id.parse::<u64>().unwrap() == category_id {
                boost_percentage += boost.percentage as f32 / 100 as f32;
            }
        }
    }

    boost_percentage
}

pub fn format_number(number: u64) -> String {
    let number_string = number.to_string();
    let mut formatted_number = String::new();

    let mut counter = 0;

    for character in number_string.chars().rev() {
        if counter == 3 {
            formatted_number.push(',');
            counter = 0;
        }

        formatted_number.push(character);
        counter += 1;
    }

    formatted_number.chars().rev().collect::<String>()
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

pub async fn send_level_up(guild: Guild, user_id: u64, current_level: i32, new_level: i32, ctx: &serenity::client::Context, msg_channel_id: u64, msg_author_name: &String) {
    let channel_id = if !guild.announce.current {
        guild.logs.levelup.unwrap_or("".to_string()).parse::<u64>().unwrap()
    } else {
        msg_channel_id
    };

    let mut lvl_msg = guild.announce.message.clone();
    lvl_msg = lvl_msg.replace("{CMB}", (new_level - current_level).to_string().as_str());
    lvl_msg = lvl_msg.replace("{LVL}", new_level.to_string().as_str());
    lvl_msg = lvl_msg.replace("{OLDLVL}", current_level.to_string().as_str());
    lvl_msg = lvl_msg.replace("{MNT}", format!("<@{}>", user_id).as_str());
    lvl_msg = lvl_msg.replace("{TAG}", format!("{}", msg_author_name).as_str());

    // send message to channel with channel_id
    let _ = ChannelId(channel_id)
        .send_message(&ctx.http, |message: &mut CreateMessage| {
            if guild.announce.ping {
                message.content(format!("<@{}>", user_id));
            }
            message.embed(|embed| {
                embed.description(lvl_msg);
                embed.color(colors::blue());
                embed
            });
            message
        })
        .await;
}
