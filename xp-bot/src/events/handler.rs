use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::{Activity, GuildId, Interaction, InteractionResponseType, Ready},
    prelude::{Context, EventHandler},
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

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
                            log::error!("Could not execute command: {:?}", why);

                            let cmd = command
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
                                            });
                                            message.ephemeral(true)            
                                        })
                                })
                                .await;

                            if let Err(why) = cmd {
                                error!("Could not execute command: {:?}", why);
                            }
                        }
                    }
                    return ();
                }
            }

            error!("Received unknown command: {:?}", command_name);
            ()
        } else if let Interaction::ModalSubmit(command) = interaction {
            let modal_data = command.data.clone();

            match modal_data.custom_id.as_str() {
                "reset_community_settings" => {
                    let guild_id = command.guild_id.unwrap();

                    let action = Guild::delete(&guild_id.0).await;

                    if action.is_err() {
                        error!("Could not reset community settings: {:?}", action.err());
                        return;
                    }
                    
                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|embed| {
                                    embed.description(
                                        "Successfully reset community settings.");
                                    embed.color(colors::green());
                                    embed
                                });
                                message.ephemeral(true)            
                            })
                    })
                    .await.unwrap();
                    
                }
                "reset_community_xp" => {
                    let guild_id = command.guild_id.unwrap();

                    let action = Guild::delete_xp(&guild_id.0).await;

                    if action.is_err() {
                        error!("Could not reset community xp: {:?}", action.err());
                        return;
                    }
                    
                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|embed| {
                                    embed.description(
                                        "Successfully reset community xp.");
                                    embed.color(colors::green());
                                    embed
                                });
                                message.ephemeral(true)            
                            })
                    })
                    .await.unwrap();
                }
                "reset_user_xp" => {
                    let experimental_extract = format!("{:?}", command
                        .data
                        .components
                        .first()
                        .unwrap()
                        .components
                        .first()
                        .unwrap());

                    // extract user id from experimental_extract
                    let user_id = experimental_extract.split("custom_id: \"reset_user_xp_input_").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
                    
                    let guild_member = GuildMember::from_id(command.guild_id.unwrap().0, user_id).await.unwrap();
                    let res = GuildMember::set_xp(command.guild_id.unwrap().0, user_id, 0, guild_member).await.unwrap();

                    if res.is_err() {
                        error!("Could not reset user xp: {:?}", res.err());
                        return;
                    }

                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|embed| {
                                    embed.description(
                                        "Successfully reset user xp.");
                                    embed.color(colors::green());
                                    embed
                                });
                                message.ephemeral(true)            
                            })
                    }).await.unwrap();
                }
                _ => {}
            };
        }
    }
}
