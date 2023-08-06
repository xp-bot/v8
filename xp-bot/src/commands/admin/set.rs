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

use crate::{commands::XpCommand, utils::colors};

pub struct SetCommand;

#[async_trait]
impl XpCommand for SetCommand {
    fn name(&self) -> &'static str {
        "setxp"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("setxp")
            .description("Set xp of a specified user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you set the xp amount of.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("amount")
                    .description("The amount of xp you want to set.")
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
        let user = command
            .data
            .options
            .first()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .clone()
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let amount = command.data.options[1]
            .value
            .as_ref()
            .unwrap()
            .clone()
            .as_i64()
            .unwrap() as u64;

        let guild_id = command.guild_id.unwrap().0;

        let guild_member = GuildMember::from_id(guild_id, user).await?;

        let _ = GuildMember::set_xp(guild_id, user, amount, guild_member).await?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.description(format!(
                                "Successfully set xp of <@{}> to {}.",
                                user, amount
                            ));
                            embed.color(colors::green());
                            embed
                        })
                    })
            })
            .await?;

        Ok(())
    }
}
