use serenity::{builder::CreateApplicationCommand, model::prelude::application_command::ApplicationCommandInteraction, prelude::Context};

use crate::{api::{user::{get_user_by_id, get_user_background_by_id}, guild::get_guild_member_by_id}, utils::imggen::generate_ranking_card};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rank")
        .description("Get info about your current level, badges and xp.")
        .create_option(|option| {
            option
                .name("user")
                .description("The user you want to check.")
                .kind(serenity::model::application::command::CommandOptionType::User)
                .required(false)
        })
}

pub async fn exec(_ctx: Context, command: ApplicationCommandInteraction) {
    let mut user_id = command.user.id.0;

    match command.data.options.first() {
        Some(option) => {
            if let Some(user) = Some(option.value.as_ref().unwrap().clone()) {
                user_id = user.as_str().unwrap().parse::<u64>().unwrap();
            }
        },
        None => {}
    }

    let user = get_user_by_id(user_id).await.unwrap();
    let guild_member = get_guild_member_by_id(command.guild_id.unwrap().0, user_id).await.unwrap();
    let background = get_user_background_by_id(user_id).await.unwrap();

    // TODO: pass user, guild_member and background to generate_ranking_card()
    generate_ranking_card();
}