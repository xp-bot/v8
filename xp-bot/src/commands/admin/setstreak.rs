use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType},
        Permissions,
    },
    prelude::Context,
};
use xp_db_connector::guild_member::{self, GuildMember};

use crate::{commands::XpCommand, utils::colors};

pub struct SetStreakCommand;

#[async_trait]
impl XpCommand for SetStreakCommand {
    fn name(&self) -> &'static str {
        "setstreak"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("setstreak")
            .description("Set the streak of a specified user.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user you set the streak of.")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("type")
                    .description("The type of streak you want to set.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("daily", "daily")
                    .add_string_choice("trivia", "trivia")
            })
            .create_option(|option| {
                option
                    .name("streak")
                    .description("The streak you want to set.")
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

        let mut guild_member: GuildMember =
            GuildMember::from_id(command.guild_id.unwrap().0, user_id).await?;

        let streak_type = command
            .data
            .options
            .get(1)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();

        let streak = command
            .data
            .options
            .get(2)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_i64()
            .unwrap() as u64;

        match streak_type {
            "daily" => guild_member.streaks.game_daily = Some(streak),
            "trivia" => guild_member.streaks.game_trivia = Some(streak),
            _ => {}
        }

        let _ = GuildMember::set_guild_member(command.guild_id.unwrap().0, user_id, guild_member)
            .await?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(
                        serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource,
                    )
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed.description(format!(
                                    "The streak of <@{}> has been set to {}.",
                                    user_id, streak
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
