use log::error;
use serenity::{
    builder::{CreateApplicationCommand, CreateEmbed},
    model::{
        application::command::CommandOptionType,
        application::interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};

use crate::utils::{math::get_required_xp, colors};
use xp_db_connector::guild_member::GuildMember;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("level")
        .description("Check how much xp is required to reach a specific level.")
        .create_option(|option| {
            option
                .name("level")
                .description("The level you want to check.")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}

pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) {
    let guild_member = GuildMember::from_id(command.guild_id.unwrap().0, command.user.id.0).await;

    if guild_member.is_err() {
        error!("Could not get guild member: {:?}", command.user.id.0);
        return ();
    }
    let guild_member = guild_member.unwrap();

    let level = command.data.options[0]
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap() as i32;
    let required_xp = get_required_xp(level);

    let result = command.create_interaction_response(&ctx.http, |response| {
        response.interaction_response_data(|message| {
            message.embed(|embed: &mut CreateEmbed| {
                embed.title(format!("Level {}", level)).description(format!(
                    "You need **{} xp** to reach level **{}**.\n You currently have **{} xp** (**{}%**).",
                    required_xp,
                    level,
                    guild_member.xp,
                    (guild_member.xp as f32 / required_xp as f32 * 100.0).round()
                )).color(colors::blue())
            })
        })
    }).await;

    if let Err(why) = result {
        error!("Could not respond to slash command: {:?}", why);
    }
}
