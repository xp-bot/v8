use std::time::Duration;

use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateComponents},
    futures::StreamExt,
    model::{
        self,
        prelude::{
            application_command::ApplicationCommandInteraction, InteractionResponseType, UserId,
        }, user::User,
    },
    prelude::Context,
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

use crate::{
    commands::XpCommand,
    utils::{
        colors,
        utils::{calc_games_bulk, eligibility_helper, GameResult},
    },
};

pub struct PartyCommand;

#[async_trait]
impl XpCommand for PartyCommand {
    fn name(&self) -> &'static str {
        "party"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("party")
            .description("Play games with a group of people.")
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

        command
            .create_interaction_response(ctx, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.title("Join the party!");
                            embed.description(format!(
                                "You need at least **2 people** to start a party and they need to join within **30 seconds**."
                            ));
                            embed.color(colors::blue())
                        });

                        // components
                        message.components(|component| {
                            component.create_action_row(|action_row| {
                                action_row.create_button(|button| {
                                    button
                                        .label("Join")
                                        .style(model::application::component::ButtonStyle::Primary)
                                        .custom_id("join_party")
                                });
                                action_row
                            })
                        });
                        message
                    })
            })
            .await?;

        let mut message = command.get_interaction_response(ctx).await?;

        let mut collector = message
            .await_component_interactions(ctx)
            .timeout(Duration::from_secs(30))
            .collect_limit(10)
            .build();

        let mut joined: Vec<UserId> = Vec::new();

        while let Some(interaction) = collector.next().await {
            if !joined.contains(&interaction.user.id) {
                joined.push(interaction.user.id);

                interaction
                    .create_interaction_response(ctx, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(format!(
                                    "**{}** joined the party!",
                                    interaction.user.name
                                ));
                                message.ephemeral(true);
                                message
                            })
                    })
                    .await?;
            } else {
                interaction.defer(&ctx.http).await?;
            }
        }

        if joined.len() < 2 {
            message
                .edit(ctx, |message| {
                    message.embed(|embed| {
                        embed.title("Party cancelled");
                        embed.description("Not enough people joined the party.");
                        embed.color(colors::red())
                    })
                })
                .await?;
            return Ok(());
        }

        let guild = Guild::from_id(command.guild_id.unwrap().0).await?;

        // calculate game xp for every participant
        let mut games: Vec<GameResult> = Vec::new();
        let mut users: Vec<User> = Vec::new();

        for user_id in joined.clone() {
            // cache users for result message
            users.push(ctx.http.get_user(user_id.0).await?);

            let mut member = GuildMember::from_id(command.guild_id.unwrap().0, user_id.0).await?;

            let result = calc_games_bulk(
                guild.values.rollXP,
                guild.values.fishXP,
                guild.values.lootXP,
            );

            member.xp += result.roll as u64 + result.fish as u64 + result.loot as u64;

            let _ = GuildMember::set_guild_member(command.guild_id.unwrap().0, user_id.0, member)
                .await?;

            games.push(result);
        }

        let winner = games
            .iter()
            .enumerate()
            .max_by_key(|(_, game)| game.roll + game.fish + game.loot)
            .unwrap()
            .0;

        message
            .edit(ctx, |message| {
                message.set_components(CreateComponents::default());

                message.embed(|embed| {
                    embed.title("Party results");
                    embed.description(format!("{} people participated. **{}** won!", games.len(), users[winner].name));

                    for i in 0..joined.len() {

                        embed.field(
                            format!("{}", users[i].name),
                            format!(
                                "**Roll**: {} xp\n**Fish**: {} xp\n**Loot**: {} xp",
                                games[i].roll, games[i].fish, games[i].loot
                            ),
                            false,
                        );
                    }

                    embed.color(colors::green())
                })
            })
            .await?;

        Ok(())
    }
}
