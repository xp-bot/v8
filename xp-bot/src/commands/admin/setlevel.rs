use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            application_command::ApplicationCommandInteraction, command::CommandOptionType,
            InteractionResponseType,
        },
        Permissions,
    },
    prelude::Context,
};
use xp_db_connector::guild_member::GuildMember;

use crate::{
    commands::XpCommand,
    utils::{colors, math::get_required_xp},
};

pub struct SetLevelCommand;

#[async_trait]
impl XpCommand for SetLevelCommand {
    fn name(&self) -> &'static str {
        "setlevel"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("setlevel")
            .description("Set the level of a specified user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you set the level of.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("level")
                    .description("The level you want to set.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
                    .min_int_value(1)
            })
            .default_member_permissions(Permissions::MANAGE_GUILD)
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let user_id = command
            .data
            .options
            .first()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let level = command
            .data
            .options
            .last()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_i64()
            .unwrap();

        let required_xp = get_required_xp(level as i32);

        let guild_member = GuildMember::from_id(command.guild_id.unwrap().into(), user_id).await?;
        let _ = GuildMember::set_xp(
            command.guild_id.unwrap().into(),
            user_id,
            required_xp as u64,
            guild_member,
        )
        .await?;

        let _ = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed.description(format!(
                                    "Set the level of <@{}> to **{}**.",
                                    user_id, level
                                ));
                                embed.color(colors::green())
                            })
                            .ephemeral(true);
                        message
                    })
            })
            .await;

        Ok(())
    }
}
