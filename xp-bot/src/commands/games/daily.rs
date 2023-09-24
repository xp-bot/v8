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
        utils::{eligibility_helper, format_number, is_cooldowned},
    },
};

pub struct DailyCommand;

#[async_trait]
impl XpCommand for DailyCommand {
    fn name(&self) -> &'static str {
        "daily"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("daily")
            .description("Claim a certain amount of xp every day!")
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

        if !eligibility_helper(command.user.id.0, &command.guild_id.unwrap().0).await {
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

        let time_now = chrono::Utc::now().timestamp() * 1000;
        let timestamp = guild_member.timestamps.game_daily.unwrap_or(0);
        let cooldown: i64 = 86400 * 1000;

        if is_cooldowned(time_now as u64, timestamp as u64, cooldown as u64) {
            let difference = time_now - timestamp as i64;
            let time_left = cooldown as i64 - difference;

            command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .embed(|embed| {
                                    embed.description(format!(
                                        "You already claimed your daily xp. \
                                        You can claim it again **{}**.",
                                        chrono_humanize::HumanTime::from(
                                            chrono::Duration::milliseconds(time_left)
                                        )
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

        let daily_xp = 250;

        let mut old_streak_msg = String::new();
        // reset streak if last daily was claimed more than 48 hours ago
        let streak = if time_now - guild_member.timestamps.game_daily.unwrap_or(0) as i64
            > 86400 * 1000 * 2
        {
            old_streak_msg = format!(
                "Your old streak was **{}**.",
                guild_member.streaks.game_daily.unwrap_or(0)
            );
            1
        } else {
            guild_member.streaks.game_daily.unwrap_or(0) + 1
        };

        let member_xp = daily_xp * streak;
        let xp_to_add = if member_xp > guild.values.maximumdailyxp as u64 {
            guild.values.maximumdailyxp as u64
        } else {
            member_xp
        };
        guild_member.xp += xp_to_add;

        guild_member.timestamps.game_daily = Some(time_now as u64);
        guild_member.streaks.game_daily = Some(streak);

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
                                "You claimed **{}** xp. Your streak is now **{}**.\n\n{}",
                                format_number(xp_to_add),
                                streak,
                                old_streak_msg
                            ));
                            embed.color(colors::green())
                        })
                    })
            })
            .await?;

        Ok(())
    }
}
