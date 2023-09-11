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
use xp_db_connector::{guild_member::GuildMember, guild::Guild};

use crate::{commands::XpCommand, utils::{colors, utils::{format_number, handle_level_roles}, math::calculate_level}};

pub struct AddCommand;

#[async_trait]
impl XpCommand for AddCommand {
    fn name(&self) -> &'static str {
        "addxp"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("addxp")
            .description("Give xp to a specified user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you want to give xp to.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("amount")
                    .description("The amount of xp you want to give.")
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

        let new_amount = guild_member.xp + amount;

        let guild = Guild::from_id(guild_id).await?;
        let new_level = calculate_level(&new_amount);

        let _ = GuildMember::set_xp(guild_id, user, &new_amount, &guild_member).await?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.description(format!(
                                "Successfully added **{}** xp to <@{}>.",
                                format_number(amount as u64), user
                            ));
                            embed.color(colors::green());
                            embed
                        }).ephemeral(true);
                        message
                    })
            })
            .await?;

        handle_level_roles(&guild, &user, &new_level, &ctx, command.guild_id.clone().unwrap().0).await;


        Ok(())
    }
}
