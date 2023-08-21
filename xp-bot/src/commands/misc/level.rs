use log::error;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    model::{
        application::command::CommandOptionType,
        application::interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};

use crate::{
    commands::XpCommand,
    utils::{colors, math::get_required_xp},
};
use xp_db_connector::guild_member::GuildMember;

pub struct LevelCommand;

#[async_trait]
impl XpCommand for LevelCommand {
    fn name(&self) -> &'static str {
        "level"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("level")
            .description("Check how much xp is required to reach a specific level.")
            .create_option(|option| {
                option
                    .name("level")
                    .description("The level you want to check.")
                    .kind(CommandOptionType::Integer)
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
            error!("Could not get guild member: {:?}", command.user.id.0);
            return Ok(());
        }
        let guild_member = guild_member?;

        let level = command.data.options[0]
            .value
            .as_ref()
            .unwrap()
            .as_i64()
            .unwrap() as i32;
        let required_xp = get_required_xp(level);

        let _ = command.create_interaction_response(&ctx.http, |response| {
        response.interaction_response_data(|message| {
            message.embed(|embed: &mut CreateEmbed| {
                embed.title(format!("Level {}", level)).description(format!(
                    "You need **{} xp** to reach level **{}**.\n You currently have **{} xp** (**{}%**).",
                    crate::utils::utils::format_number(required_xp as u64),
                    level,
                    crate::utils::utils::format_number(guild_member.xp as u64),
                    (guild_member.xp as f32 / required_xp as f32 * 100.0).round()
                )).color(colors::blue())
            })
        })
    }).await?;

        Ok(())
    }
}
