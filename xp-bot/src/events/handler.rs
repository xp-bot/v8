use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::{Activity, GuildId, Interaction, InteractionResponseType, Ready},
    prelude::{Context, EventHandler},
};

use crate::{commands, utils::colors};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            info!(
                "{} is connected on shard {}/{}!",
                ready.user.name,
                shard[0] + 1,
                shard[1]
            );
        } else {
            info!("Connected on shard");
        }

        // set activity
        ctx.set_activity(Activity::listening("xp-bot.net")).await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Cache is ready!");

        // register slash commands

        for guild in guilds {
            let commands = GuildId::set_application_commands(&guild, &ctx.http, |commands| {
                for command in commands::COMMANDS {
                    commands.create_application_command(|c| command.register(c));
                }
                commands
            })
            .await;

            if let Err(why) = commands {
                error!("Could not register commands for guild {}: {:?}", guild, why);
            }

            info!("Registered commands for guild {}", guild);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // handle slash commands
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = command.data.name.as_str();

            for cmd in commands::COMMANDS {
                if cmd.name() == command_name {
                    let result = cmd.exec(&ctx, &command).await;

                    match result {
                        Ok(_) => {}
                        Err(why) => {
                            command
                                .create_interaction_response(&ctx.http, |response| {
                                    response
                                        .kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|message| {
                                            message.embed(|embed| {
                                                embed.title("Error");
                                                embed.description(
                                                    "An error occured while executing the command.\nIf this error persists, please join our [support server](https://discord.xp-bot.net).");
                                                embed.color(colors::red());
                                                embed
                                            })                                         
                                        })
                                })
                                .await
                                .unwrap();
                            error!("Could not execute command: {:?}", why);
                        }
                    }
                    return ();
                }
            }

            error!("Received unknown command: {:?}", command_name);
            ()
        }
    }
}
