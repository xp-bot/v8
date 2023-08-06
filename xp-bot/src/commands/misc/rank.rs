use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};

use crate::commands::XpCommand;

pub struct RankCommand;

#[async_trait]
impl XpCommand for RankCommand {
    fn name(&self) -> &'static str {
        "rank"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
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

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut _user_id = command.user.id.0;

        match command.data.options.first() {
            Some(option) => {
                if let Some(user) = Some(option.value.as_ref().unwrap().clone()) {
                    _user_id = user.as_str().unwrap().parse::<u64>().unwrap();
                }
            }
            None => {}
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!(
                            "https://bot-api.xp-bot.net/rank/{}/{}?cache={}",
                            command.guild_id.unwrap().0,
                            _user_id,
                            chrono::Utc::now().timestamp()
                        ))
                    })
            })
            .await?;

        Ok(())
    }
}
