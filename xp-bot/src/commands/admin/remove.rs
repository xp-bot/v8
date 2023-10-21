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
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

use crate::{
    commands::XpCommand,
    utils::{
        colors,
        math::calculate_level,
        utils::{format_number, handle_level_roles},
    },
};

pub struct RemoveCommand;

#[async_trait]
impl XpCommand for RemoveCommand {
    fn name(&self) -> &'static str {
        "removexp"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("removexp")
            .description("Remove xp of a user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you want to remove xp from.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("amount")
                    .description("The amount of xp you want to remove.")
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
            .parse::<u64>()?;

        let amount = command.data.options[1]
            .value
            .as_ref()
            .unwrap()
            .clone()
            .as_i64()
            .unwrap() as u64;

        let guild_id = command.guild_id.unwrap().0;

        let guild_member = GuildMember::from_id(guild_id, user).await?;

        let new_amount = guild_member.xp - amount;

        let guild = Guild::from_id(guild_id).await?;
        let new_level = calculate_level(&new_amount);

        let _ = GuildMember::set_xp(guild_id, user, &new_amount, &guild_member).await?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed.description(format!(
                                    "Successfully removed **{}** xp from <@{}>.",
                                    format_number(amount as i64),
                                    user
                                ));
                                embed.color(colors::green());
                                embed
                            })
                            .ephemeral(true);
                        message
                    })
            })
            .await?;

        handle_level_roles(
            &guild,
            &user,
            &new_level,
            &ctx,
            command.guild_id.clone().unwrap().0,
        )
        .await;

        Ok(())
    }
}
