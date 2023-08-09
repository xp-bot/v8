use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

use crate::{
    commands::XpCommand,
    utils::{
        colors,
        utils::{eligibility_helper, is_cooldowned},
    },
};
use rand::Rng;

pub struct FishCommand;

#[async_trait]
impl XpCommand for FishCommand {
    fn name(&self) -> &'static str {
        "fish"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.name("fish").description("Go fishing and find out!")
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let guild = Guild::from_id(command.guild_id.unwrap().0).await?;

        if !guild.modules.games {
            command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed.title("Games module disabled");
                                embed.description(
                                    format!("This module is disabled on this server. \
                                    An administrator can enable it [here](https://xp-bot.net/me/servers/{}/modules).", command.guild_id.unwrap().0).as_str(),
                                );
                                embed.color(colors::red())
                            }).ephemeral(true);
                            message
                        })
                })
                .await?;

            return Ok(());
        }

        if !eligibility_helper(command.user.id.0).await {
            command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed.title("Vote required");
                                embed.description(
                                    "You need to vote for the bot to use this command. \
                                    You can vote [here](https://top.gg/bot/706935674800177193/vote).",
                                );
                                embed.field("Don't want to vote?", "You can also get premium [here](https://xp-bot.net/premium).", false);
                                embed.color(colors::red())
                            }).ephemeral(true);
                            message
                        })
                })
                .await?;

            return Ok(());
        }

        let mut guild_member =
            GuildMember::from_id(command.guild_id.unwrap().0, command.user.id.0).await?;

        // check cooldowns
        let time_now = chrono::Utc::now().timestamp() * 1000;
        let timestamp = guild_member.timestamps.game_fish;
        let cooldown = guild.values.gamecooldown * 1000;

        if is_cooldowned(time_now as u64, timestamp.unwrap() as u64, cooldown as u64) {
            let difference = time_now - timestamp.unwrap() as i64;
            let time_left = cooldown as i64 - difference;

            command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .embed(|embed| {
                                    embed.description(format!(
                                        "You need to wait **{} seconds** before you can fish again.",
                                        time_left / 1000
                                    ));
                                    embed.color(colors::red())
                                })
                                .ephemeral(true);
                            message
                        })
                })
                .await?;

            return Ok(());
        }

        // assign xp
        let random_xp = rand::thread_rng().gen_range(1..=guild.values.fishXP);

        guild_member.xp += random_xp as u64;

        let _ = GuildMember::set_guild_member(
            command.guild_id.unwrap().0,
            command.user.id.0,
            guild_member.clone(),
        )
        .await?;

        // set new cooldown
        guild_member.timestamps.game_fish = Some(time_now as u64);
        let _ = GuildMember::set_guild_member(
            command.guild_id.unwrap().0,
            command.user.id.0,
            guild_member,
        )
        .await?;

        command
            .create_interaction_response(ctx, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.description(format!(
                                ":fishing_pole_and_fish: | You caught a fish and gained **{}** xp!",
                                random_xp
                            ));
                            embed.color(colors::green())
                        })
                    })
            })
            .await?;

        Ok(())
    }
}
