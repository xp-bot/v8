use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{
        application_command::ApplicationCommandInteraction, command::CommandOptionType,
    },
    prelude::Context,
};
use xp_db_connector::guild_member::GuildMember;

use crate::commands::XpCommand;

pub struct DistanceCommand;

#[async_trait]
impl XpCommand for DistanceCommand {
    fn name(&self) -> &'static str {
        "distance"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("distance")
            .description("See how far you are from another user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you want to check.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let guild_member =
            GuildMember::from_id(command.guild_id.unwrap().0, command.user.id.0).await;

        if guild_member.is_err() {
            log::error!("Could not get guild member: {:?}", command.user.id.0);
            return Ok(());
        }

        let guild_member = guild_member?;

        let user_id = command.data.options[0]
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let other_guild_member = GuildMember::from_id(command.guild_id.unwrap().0, user_id).await;

        if other_guild_member.is_err() {
            log::error!("Could not get guild member: {:?}", user_id);
            return Ok(());
        }

        let other_guild_member = other_guild_member?;
        let required_xp = other_guild_member.xp as i64 - guild_member.xp as i64;

        let username = ctx.http.get_user(user_id).await.unwrap().name.clone();

        let description_str = if required_xp < 0 {
            format!(
                "You are **{} xp** ahead of **{}**.",
                crate::utils::utils::format_number(required_xp.abs()),
                username
            )
        } else {
            format!(
                "You are **{} xp** behind **{}**.",
                crate::utils::utils::format_number(required_xp),
                username
            )
        };

        let mut embed = CreateEmbed::default();
        embed.title("Distance");
        embed.color(crate::utils::colors::blue());
        embed.description(description_str.as_str());

        let _ = command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message.add_embed(embed);
                    message
                })
            })
            .await?;

        Ok(())
    }
}
