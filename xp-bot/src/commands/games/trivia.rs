use std::time::Duration;

use rand::seq::SliceRandom;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{
        self,
        prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    },
    prelude::Context,
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

use crate::{
    commands::XpCommand,
    utils::{
        colors,
        opentdb::OpenTriviaDB,
        utils::{eligibility_helper, is_cooldowned},
    },
};

pub struct TriviaCommand;

#[async_trait]
impl XpCommand for TriviaCommand {
    fn name(&self) -> &'static str {
        "trivia"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("trivia")
            .description("Test your knowledge and earn xp!")
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let guild = Guild::from_id(command.guild_id.unwrap().0).await?;

        if !guild.modules.trivia {
            command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed.title("Trivia module disabled");
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

        let time_now = chrono::Utc::now().timestamp() * 1000;
        let timestamp = guild_member.timestamps.game_trivia.unwrap_or(0);
        let cooldown = guild.values.triviacooldown * 1000;

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
                                    embed.description(
                                        format!(
                                            "You need to wait **{}** seconds before using this command again.",
                                            time_left / 1000
                                        )
                                        .as_str(),
                                    );
                                    embed.color(colors::red())
                                })
                                .ephemeral(true);
                            message
                        })
                })
                .await?;

            return Ok(());
        }

        let question = OpenTriviaDB::get_question().await?;

        let xp_multiplier = match question.results.first().unwrap().difficulty.as_str() {
            "easy" => 1,
            "medium" => 2,
            "hard" => 3,
            _ => 1,
        };
        let xp = guild.values.triviaxp * xp_multiplier;

        // randomly put all answers in a vector and remember the index of the correct answer
        let mut answers = vec![
            question.results.first().unwrap().correct_answer.clone(),
            question.results.first().unwrap().incorrect_answers[0].clone(),
            question.results.first().unwrap().incorrect_answers[1].clone(),
            question.results.first().unwrap().incorrect_answers[2].clone(),
        ];
        answers.shuffle(&mut rand::thread_rng());
        let correct_answer_index = answers
            .iter()
            .position(|x| x == &question.results.first().unwrap().correct_answer)
            .unwrap();

        command
            .create_interaction_response(ctx, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.title(question.results.first().unwrap().question.clone());
                            embed.description(format!(
                                "You have **30** seconds to answer the following question."
                            ));
                            embed.field(
                                "Answers",
                                format!(
                                    "1. {}\n2. {}\n3. {}\n4. {}",
                                    answers[0], answers[1], answers[2], answers[3]
                                ),
                                false,
                            );
                            embed.color(colors::blue())
                        });

                        // components
                        message.components(|component| {
                            component.create_action_row(|action_row| {
                                action_row.create_button(|button| {
                                    button
                                        .label("1")
                                        .style(model::application::component::ButtonStyle::Primary)
                                        .custom_id("1")
                                });
                                action_row.create_button(|button| {
                                    button
                                        .label("2")
                                        .style(model::application::component::ButtonStyle::Primary)
                                        .custom_id("2")
                                });
                                action_row.create_button(|button| {
                                    button
                                        .label("3")
                                        .style(model::application::component::ButtonStyle::Primary)
                                        .custom_id("3")
                                });
                                action_row.create_button(|button| {
                                    button
                                        .label("4")
                                        .style(model::application::component::ButtonStyle::Primary)
                                        .custom_id("4")
                                });
                                action_row
                            })
                        });
                        message
                    })
            })
            .await?;

        let message = command.get_interaction_response(ctx).await?;

        let collector = message
            .await_component_interaction(&ctx)
            .author_id(command.user.id)
            .timeout(Duration::from_secs(30))
            .await;

        match collector {
            Some(interaction) => {
                interaction.defer(&ctx.http).await?;

                let mut correct = false;

                if interaction.user.id == command.user.id {
                    if interaction.data.custom_id == correct_answer_index.to_string() {
                        correct = true;
                    }
                }

                if correct {
                    guild_member.xp += xp as u64;
                    guild_member.timestamps.game_trivia = Some(time_now as u64);
                    guild_member.streaks.game_trivia = Some(guild_member.streaks.game_trivia.unwrap_or(0) + 1);

                    let _ = GuildMember::set_guild_member(
                        command.guild_id.unwrap().0,
                        command.user.id.0,
                        guild_member,
                    )
                    .await?;

                    command
                        .create_followup_message(&ctx.http, |response| {
                            response
                                .embed(|embed| {
                                    embed.description(format!(
                                        "You answered correctly and received **{}** XP.",
                                        xp
                                    ));
                                    embed.color(colors::green())
                                })
                                .ephemeral(true)
                        })
                        .await?;
                } else {
                    command
                        .create_followup_message(&ctx.http, |response| {
                            response
                                .embed(|embed| {
                                    embed.description("You answered incorrectly.");
                                    embed.color(colors::red())
                                })
                                .ephemeral(true)
                        })
                        .await?;
                }

                return Ok(());
            }
            None => {
                command
                    .edit_original_interaction_response(ctx, |response| {
                        response.embed(|embed| {
                            embed.description("You didn't answer in time.");
                            embed.color(colors::red())
                        })
                    })
                    .await?;

                return Ok(());
            }

        }
    }
}
