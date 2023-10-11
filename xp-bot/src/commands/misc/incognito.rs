use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};
use xp_db_connector::guild_member::GuildMember;

use crate::{commands::XpCommand, utils::colors};

pub struct IncognitoCommand;

#[async_trait]
impl XpCommand for IncognitoCommand {
    fn name(&self) -> &'static str {
        "incognito"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("incognito")
            .description("Set wether you want your XP profile to be publicly visible or not.")
            .create_option(|option| {
                option
                    .name("enabled")
                    .description(
                        "Set wether you want your XP profile to be publicly visible or not.",
                    )
                    .kind(serenity::model::application::command::CommandOptionType::Boolean)
                    .required(true)
            })
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let enabled = command
            .data
            .options
            .first()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_bool()
            .unwrap();

        let guild_id = command.guild_id.unwrap().0;

        let mut guild_member = GuildMember::from_id(guild_id, command.user.id.0).await?;

        guild_member.settings.incognito = Some(enabled);

        let _ = GuildMember::set_guild_member(guild_id, command.user.id.0, guild_member).await?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed.description(format!(
                                    "Your incognito status has been set to **{}**.",
                                    if enabled { "enabled" } else { "disabled" }
                                ));
                                embed.color(colors::green());
                                embed
                            })
                            .ephemeral(true);
                        message
                    })
            })
            .await?;

        Ok(())
    }
}
