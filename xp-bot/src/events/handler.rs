use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::{Activity, GuildId, Interaction, InteractionResponseType, Ready, Message},
    prelude::{Context, EventHandler},
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember};

use crate::{commands, utils::{colors, utils::{is_cooldowned, self}}};

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

    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0;
        let guild_id = msg.guild_id.unwrap().0;

        // check if message is longer than 5 characters
        if msg.content.len() < 5 {
            return ();
        }

        // get database data
        let guild = Guild::from_id(guild_id).await.unwrap();
        let mut member = GuildMember::from_id(guild_id, user_id).await.unwrap();

        // check if messagexp module is enabled
        if !guild.modules.messagexp {
            return ();
        }

        // check if user is on cooldown
        let last_timestamp = member.timestamps.message_cooldown.unwrap_or(0);
        let timestamp = chrono::Utc::now().timestamp() * 1000;
        if is_cooldowned(timestamp as u64, last_timestamp, guild.values.messagecooldown as u64 * 1000) {
            return ();
        }

        // check for ignored roles, channels or categories
        let channel_id = msg.channel_id.0;

        let category_id = match msg.channel_id.to_channel(&ctx).await.unwrap() {
            serenity::model::channel::Channel::Guild(channel) => {
                channel.parent_id.unwrap().0
            }
            _ => 0,
        };

        let role_ids = msg
            .member
            .unwrap()
            .roles
            .iter()
            .map(|role| role.0)
            .collect::<Vec<u64>>();

        for ignored_role in guild.ignored.roles {
            if role_ids.contains(&ignored_role.parse::<u64>().unwrap().to_owned()) {
                return ();
            }
        }

        if guild
            .ignored
            .channels
            .unwrap()
            .contains(&channel_id.to_owned().to_string())
        {
            return ();
        }

        if guild
            .ignored
            .categories
            .unwrap()
            .contains(&category_id.to_owned().to_string())
        {
            return ();
        }

        // calculate boost percentage
        let guild = Guild::from_id(guild_id).await.unwrap();
        let boost_percentage = utils::calculate_total_boost_percentage_by_ids(
            guild.clone(),
            role_ids,
            channel_id,
            Some(category_id),
        );

        // calculate xp
        let xp = (guild.values.messagexp as f32 * (boost_percentage + 1.0)) as u32;

        // check if user leveled up, dont send if user is incognito

        // add xp to user
        member.xp += xp as u64;

        // set new cooldown
        member.timestamps.message_cooldown = Some(timestamp as u64);

        // update database
        let _ = GuildMember::set_xp(guild_id, user_id, member.xp, member).await;
    }
}
