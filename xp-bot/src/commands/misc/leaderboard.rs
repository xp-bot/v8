use log::error;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::InteractionResponseType,
    },
    prelude::Context,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Check who's most active on this server.")
}

pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) {
    let url = format!(
        "https://xp-bot.net/lb/{}",
        command.guild_id.unwrap().0
    );

    let result = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(url))
        })
        .await;

    if let Err(why) = result {
        error!("Could not respond to command: {:?}", why);
    }
}
