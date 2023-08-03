use log::error;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::InteractionResponseType,
    },
    prelude::Context,
};

use crate::commands::XpCommand;

pub struct LeaderboardCommand;

#[async_trait]
impl XpCommand for LeaderboardCommand {
    fn name(&self) -> &'static str {
        "leaderboard"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("leaderboard")
            .description("Check who's most active on this server.")
    }

    async fn exec(&self, ctx: &Context, command: &ApplicationCommandInteraction) {
        let url = format!("https://xp-bot.net/lb/{}", command.guild_id.unwrap().0);

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
}
