use log::{error, info};
use serenity::{
    async_trait,
    model::{prelude::{Activity, GuildId, Interaction, InteractionResponseType, Ready, Message, Reaction, ChannelId, component::ButtonStyle, ReactionType, Member, RoleId}, voice::VoiceState},
    prelude::{Context, EventHandler},
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember, user::User};

use crate::{commands, utils::{colors, utils::{is_cooldowned, self, send_level_up, handle_level_roles, conform_xpc}, math::calculate_level}};

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
                    
                    let mut guild_member = GuildMember::from_id(command.guild_id.unwrap().0, user_id).await.unwrap();

                    guild_member = conform_xpc(guild_member, &ctx, &command.guild_id.unwrap().0, &user_id).await;
                    let res = GuildMember::set_xp(command.guild_id.unwrap().0, user_id, &0, &guild_member).await.unwrap();

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

    // XP welcome message when it gets invited to a server
    async fn guild_create(&self, ctx: Context, guild: serenity::model::guild::Guild, is_new: bool) {
        if !is_new {
            return ();
        }

        // get first text channel and send a message
        let channels = guild.channels;

        let mut channel_id = 0;
        for channel in channels {
            let channel_type = channel.1;

            match channel_type.guild() {
                Some(channel) => {
                    if channel.kind == serenity::model::channel::ChannelType::Text {
                        channel_id = channel.id.0;
                        break;
                    }
                }
                None => {}
            };
        };

        ChannelId(channel_id).send_message(&ctx.http, |message| {
            message.embed(|embed| {
                embed.title("Welcome to XP 👋");
                embed.description("We are happy, that you chose XP for your server!\nXP is a leveling bot, that allows you to reward your members for their activity.\nIf you need help, feel free to join our [support server](https://discord.xp-bot.net)!");
                embed.field(
                    "Read our tutorials", 
                    format!("- {}\n- {}\n- {}\n- {}", 
                    "[Roles & Boosts](https://xp-bot.net/blog/guide_roles_and_boosts_1662020313458)",
                    "[Announcements](https://xp-bot.net/blog/guide_announcements_1660342055312)",
                    "[Values](https://xp-bot.net/blog/guide_values_1656883362214)",
                    "[XP, Moderation & Game Modules](https://xp-bot.net/blog/guide_xp__game_modules_1655300944128)"
                    ), false
                );
                embed.color(colors::blue());
                embed
            });
            message.components(
                |components| components
                    .create_action_row(|action_row| action_row
                        .create_button(|button| button
                            .label("Server Dashboard")
                            .style(ButtonStyle::Link)
                            .emoji(ReactionType::Unicode("🛠️".to_string()))
                            .url(format!("https://xp-bot.net/servers/{}", &guild.id.to_string()))
                        )
                        .create_button(|button| button
                            .label("Account Settings")
                            .style(ButtonStyle::Link)
                            .emoji(ReactionType::Unicode("🙋".to_string()))
                            .url("https://xp-bot.net/me")
                        )
                        .create_button(|button| button
                            .label("Premium")
                            .style(ButtonStyle::Link)
                            .emoji(ReactionType::Unicode("👑".to_string()))
                            .url("https://xp-bot.net/premium")
                        )
                        .create_button(|button| button
                            .label("Privacy Policy")
                            .style(ButtonStyle::Link)
                            .emoji(ReactionType::Unicode("🔖".to_string()))
                            .url("https://xp-bot.net/legal/privacy")
                        )
                    )
            )
        }).await.unwrap();
    }

    async fn guild_member_addition(&self, ctx: Context, mut new_member: Member) {
        let guild = Guild::from_id(new_member.guild_id.0).await.unwrap();

        // get role that is assigned to level -1
        let autorole = guild.levelroles.iter().find(|role| role.level == -1);
        
        if autorole.is_none() {
            return ();
        }

        let autorole = autorole.unwrap();

        log::info!("Assigning autorole {} to {}", autorole.id, new_member.user.name);

        // assign autorole
        let _ = new_member.add_role(&ctx.http, RoleId(autorole.id.parse::<u64>().unwrap())).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return ();
        }

        let user_id = msg.author.id.0;
        let guild_id = msg.guild_id.clone().unwrap().0;

        // check if message is longer than 5 characters
        if msg.content.len() < 5 {
            return ();
        }

        // get database data
        let guild = Guild::from_id(guild_id).await.unwrap();
        let mut member = GuildMember::from_id(guild_id, user_id).await.unwrap();

        // check if messagexp module is enabled
        if guild.modules.messagexp {
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
            let current_level = calculate_level(&member.xp);
            let new_level = calculate_level(&(member.xp + xp as u64));

            if new_level > current_level {
                handle_level_roles(&guild.clone(), &user_id, &new_level, &ctx, msg.guild_id.clone().unwrap().0).await;
                
                if !member.settings.incognito.unwrap_or(false) {
                    send_level_up(guild.clone(), user_id, current_level, new_level, &ctx, msg.channel_id.0.clone(), &msg.author.name).await;
                }
            }

            // add xp to user
            member.xp += xp as u64;

            // set new cooldown
            member.timestamps.message_cooldown = Some(timestamp as u64);

            member = conform_xpc(member, &ctx, &guild_id, &msg.author.id.0).await;
            // update database
            let _ = GuildMember::set_xp(guild_id, user_id, &member.xp, &member).await;
        }

        /*
            Handle autonick logic
            > Autonick is a module, that adds [Lvl. {level}] to the nickname of a user.
            > If use_prefix is enabled, [Lvl. {level}] will be added to the beginning of the nickname.
            > If show_string is enabled, only [{level}] will be added to the end of the nickname.
        */

        // check if user is a bot
        if msg.author.bot {
            return ();
        }

        // get users level
        let level = calculate_level(&member.xp);

        // check for config modules
        let use_prefix = guild.modules.autonickuseprefix;
        let show_string = guild.modules.autonickshowstring;

        // create new nickname
        let regex = regex::Regex::new(r"\s*\[.*?\]\s*").unwrap();
        let mut current_nick = ctx
            .http
            .get_member(guild_id, user_id)
            .await
            .unwrap()
            .nick
            .unwrap_or(msg.author.name.clone());

        // replace previous autonick
        current_nick = regex.replace(&current_nick, "").to_string();

        // if autonick is disabled, reset nickname to previous nickname and return
        if !guild.modules.autonick {
            let _ = ctx
                .http
                .get_member(guild_id, user_id)
                .await
                .unwrap()
                .edit(&ctx.http, |edit| edit.nickname(current_nick.clone()))
                .await;
            return ();
        }

        let new_nick = {
            let lvl_str = if !show_string {
                format!("[{}]", level)
            } else {
                format!("[Lvl. {}]", level)
            };

            let new_nick = if use_prefix {
                format!("{} {}", lvl_str, current_nick)
            } else {
                format!("{} {}", current_nick, lvl_str)
            };
            new_nick
        };

        // check if nickname is too long
        if new_nick.len() > 32 {
            log::error!("Nickname of {} is too long", msg.author.name);
            return ();
        }

        // update nickname
        let _ = ctx
            .http
            .get_member(guild_id, user_id)
            .await
            .unwrap()
            .edit(&ctx.http, |edit| edit.nickname(new_nick.clone()))
            .await;

        return ();
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let user_id = add_reaction.user_id.unwrap().0;
        let guild_id = add_reaction.guild_id.unwrap().0;

        // get database data
        let guild = Guild::from_id(guild_id).await.unwrap();
        let mut member = GuildMember::from_id(guild_id, user_id).await.unwrap();

        // check if reactionxp module is enabled
        if !guild.modules.reactionxp {
            return ();
        }

        // check for ignored roles, channels or categories
        let channel_id = add_reaction.channel_id.0;

        let category_id = match add_reaction.channel_id.to_channel(&ctx).await.unwrap() {
            serenity::model::channel::Channel::Guild(channel) => {
                channel.parent_id.unwrap().0
            }
            _ => 0,
        };

        let role_ids = add_reaction
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
        let xp = (guild.values.reactionxp as f32 * (boost_percentage + 1.0)) as u32;

        // check if user leveled up, dont send if user is incognito
        let current_level = calculate_level(&member.xp);
        let new_level = calculate_level(&(member.xp + xp as u64));

        if new_level > current_level {
            let username = ctx
                .http
                .get_user(user_id)
                .await
                .unwrap()
                .name
                .to_owned();

            handle_level_roles(&guild.clone(), &user_id, &new_level, &ctx, add_reaction.guild_id.unwrap().0).await;
        
            if !member.settings.incognito.unwrap_or(false) {
                send_level_up(guild,
                    user_id,
                    current_level,
                    new_level,
                    &ctx,
                    add_reaction.channel_id.0,
                    &username,
                ).await;
            }
        }

        // add xp to user
        member.xp += xp as u64;
        member = conform_xpc(member, &ctx, &guild_id, &add_reaction.user_id.unwrap().0).await;

        // update database
        let _ = GuildMember::set_xp(guild_id, user_id, &member.xp, &member).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        // check if the event is a join, leave or move event
        if old.is_none() && new.channel_id.is_some() {
            Handler::voice_join(ctx, new.guild_id.unwrap(), &new).await;
        } else if old.is_some() && new.channel_id.is_none() {
            Handler::voice_leave(ctx, new.guild_id.unwrap(), old, new).await;
        } else if old.is_some() && new.channel_id.is_some() {
            Handler::voice_move(ctx, new.guild_id.unwrap(), new).await;
        }
    }
}

impl Handler {
    pub async fn voice_join(_ctx: Context, guild_id: GuildId, joined: &VoiceState) {
        let timestamp = chrono::Utc::now().timestamp() * 1000;
        let mut user = User::from_id(joined.user_id.0).await.unwrap();
        let guild = Guild::from_id(guild_id.0).await.unwrap();

        // check if voice module is enabled
        if !guild.modules.voicexp {
            return ();
        }

        // check if voice channel is ignored
        if guild.ignored.channels.unwrap().contains(&joined.channel_id.unwrap().0.to_string()) {
            return ();
        }

        // set new timestamp
        user.timestamps.join_voicechat = Some(timestamp as u64);

        // update database
        let _ = User::set(joined.user_id.0, user).await;
    }

    pub async fn voice_leave(ctx: Context, guild_id: GuildId, old: Option<VoiceState>, left: VoiceState) {
        let timestamp = chrono::Utc::now().timestamp() * 1000;
        let mut user = User::from_id(left.user_id.0).await.unwrap();
        let mut member = GuildMember::from_id(guild_id.0, left.user_id.0).await.unwrap();
        let guild = Guild::from_id(guild_id.0).await.unwrap();
        let log_channel_id = guild.clone().logs.voicetime;

        // check if voice module is enabled
        if !guild.modules.voicexp {
            return ();
        }

        // check if voice channel is ignored
        if guild.clone().ignored.channels.unwrap().contains(&old.clone().unwrap().channel_id.unwrap().0.to_string()) {
            return ();
        }

        // calculate time in voice chat
        let last_timestamp = user.timestamps.join_voicechat.unwrap_or(0);
        let time_in_voicechat = (timestamp - last_timestamp as i64 - guild.values.voicejoincooldown as i64 * 1000) / 1000;
        let time_in_voicechat = if time_in_voicechat < 0 { 0 } else { time_in_voicechat };

        let old = old.unwrap();
        // calculate boost percentage 
        let boost_percentage = utils::calculate_total_boost_percentage_by_ids(
            guild.clone(),
            left.member.unwrap().roles.iter().map(|role| role.0).collect::<Vec<u64>>(),
            old.channel_id.unwrap().0,
            Some(match old.channel_id.unwrap().to_channel(&ctx).await.unwrap() {
                serenity::model::channel::Channel::Guild(channel) => {
                    channel.parent_id.unwrap().0
                }
                _ => 0,
            }),
        );

        // calculate xp
        let xp = ((guild.values.voicexp as f32 * (time_in_voicechat as f32 / 60.)) * (boost_percentage + 1.0)) as u32;

        // check if user leveled up, dont send if user is incognito
        let current_level = calculate_level(&member.xp);
        let new_level = calculate_level(&(member.xp + xp as u64));

        if new_level > current_level {
            let username = ctx
                .http
                .get_user(left.user_id.0)
                .await
                .unwrap()
                .name
                .to_owned();

            handle_level_roles(&guild.clone(), &left.user_id.0, &new_level, &ctx, old.guild_id.unwrap().0).await;

            if !member.settings.incognito.unwrap_or(false) {
                send_level_up(guild.clone(),
                    left.user_id.0,
                    current_level,
                    new_level,
                    &ctx,
                    old.channel_id.unwrap().0,
                    &username,
                ).await;
            }
        }

        // send summary of voice time
        if guild.logs.voicetime.clone().is_some() {
            let voice_time = (timestamp - last_timestamp as i64) / 1000;

            // make it days, hours, minutes, seconds
            let days = voice_time / 86400;
            let hours = (voice_time - days * 86400) / 3600;
            let minutes = (voice_time - days * 86400 - hours * 3600) / 60;
            let seconds = voice_time - days * 86400 - hours * 3600 - minutes * 60;

            let time_string = if days > 0 {
                format!(
                    "**{}** days, **{}** hours, **{}** minutes, **{}** seconds",
                    days, hours, minutes, seconds
                )
            } else if hours > 0 {
                format!(
                    "**{}** hours, **{}** minutes, **{}** seconds",
                    hours, minutes, seconds
                )
            } else if minutes > 0 {
                format!("**{}** minutes, **{}** seconds", minutes, seconds)
            } else {
                format!("**{}** seconds", seconds)
            }; 

            // calculate level difference
            let current_level = calculate_level(&member.xp);
            let new_level = calculate_level(&(member.xp + xp as u64));
            let level_difference = new_level - current_level;

            let requested_user = ctx.http.get_user(left.user_id.0).await.unwrap().name.clone();

            // send message
            let _ = ChannelId(log_channel_id.clone().unwrap().parse::<u64>().unwrap())
                .send_message(&ctx.http, |message| {
                    message.embed(|embed| {
                        embed.title(format!(
                            "{}'s voicetime",
                            requested_user
                            
                        ));
                        embed.description(time_string);

                        if level_difference > 0 {
                            embed.field(
                                "Level",
                                format!("**{} → {}**", crate::utils::utils::format_number(current_level as u64), crate::utils::utils::format_number(new_level as u64)),
                                true,
                            );
                        }

                        embed.field("XP", crate::utils::utils::format_number(xp as u64), true);
                        embed.field("", "", true);
                        embed.color(colors::blue());
                        embed
                    })
                })
                .await;
        }

        // add xp to user
        member.xp += xp as u64;
        member = conform_xpc(member, &ctx, &guild_id.0, &left.user_id.0).await;

        // update database
        let _ = GuildMember::set_xp(guild_id.0, left.user_id.0, &member.xp, &member).await;

        // invalidate timestamp
        user.timestamps.join_voicechat = None;
        let _ = User::set(left.user_id.0, user).await;
    }

    /*
        IMPORTANT: Subject to change, we will build a better implementation in the future
        TODO: voice_move should be refactored in the future, as it's basically just code duplication
    */ 
    pub async fn voice_move(ctx: Context, guild_id: GuildId, moved: VoiceState) {
        // (!) can also be mute/unmute, deafen/undeafen or self mute/unmute, self deafen/undeafen

        // check if moved to voicechat is the guilds afk channel
        let afk_channel_id = match ctx.cache.guild(guild_id) {
            Some(guild) => guild.afk_channel_id,
            None => {
                ctx.http
                    .get_guild(guild_id.0)
                    .await
                    .unwrap()
                    .afk_channel_id
            },
        };

        let guild = Guild::from_id(guild_id.0).await.unwrap();

        // check if moved to voicechat is the guilds afk channel or ignore channel
        if moved.channel_id.unwrap().0 == afk_channel_id.unwrap().0 || guild.clone().ignored.channels.unwrap().contains(&moved.channel_id.unwrap().0.to_string()) {
            let timestamp = chrono::Utc::now().timestamp() * 1000;
            let mut user = User::from_id(moved.user_id.0).await.unwrap();
            let mut member = GuildMember::from_id(guild_id.0, moved.user_id.0).await.unwrap();
            let log_channel_id = guild.clone().logs.voicetime;

            // check if voice module is enabled
            if !guild.modules.voicexp {
                return ();
            }

            // check if voice channel is ignored
            if guild.clone().ignored.channels.unwrap().contains(&moved.channel_id.unwrap().0.to_string()) {
                return ();
            }

            // calculate time in voice chat
            let last_timestamp = user.timestamps.join_voicechat.unwrap_or(0);
            let time_in_voicechat = (timestamp - last_timestamp as i64 - guild.values.voicejoincooldown as i64 * 1000) / 1000;
            let time_in_voicechat = if time_in_voicechat < 0 { 0 } else { time_in_voicechat };

            // calculate boost percentage
            let boost_percentage = utils::calculate_total_boost_percentage_by_ids(
                guild.clone(),
                moved.member.unwrap().roles.iter().map(|role| role.0).collect::<Vec<u64>>(),
                moved.channel_id.unwrap().0,
                Some(match moved.channel_id.unwrap().to_channel(&ctx).await.unwrap() {
                    serenity::model::channel::Channel::Guild(channel) => {
                        channel.parent_id.unwrap().0
                    }
                    _ => 0,
                }),
            );

            // calculate xp
            let xp = ((guild.values.voicexp as f32 * (time_in_voicechat as f32 / 60.)) * (boost_percentage + 1.0)) as u32;

            // check if user leveled up, dont send if user is incognito
            let current_level = calculate_level(&member.xp);
            let new_level = calculate_level(&(member.xp + xp as u64));

            if new_level > current_level {
                let username = ctx
                    .http
                    .get_user(moved.user_id.0)
                    .await
                    .unwrap()
                    .name
                    .to_owned();

                handle_level_roles(&guild.clone(), &moved.user_id.0, &new_level, &ctx, moved.guild_id.unwrap().0).await;

                if !member.settings.incognito.unwrap_or(false) {
                    send_level_up(guild.clone(),
                        moved.user_id.0,
                        current_level,
                        new_level,
                        &ctx,
                        moved.channel_id.unwrap().0,
                        &username,
                    ).await;
                }
            }

            // send summary of voice time
            if guild.logs.voicetime.clone().is_some() {
                let voice_time = (timestamp - last_timestamp as i64) / 1000;

                // make it days, hours, minutes, seconds
                let days = voice_time / 86400;
                let hours = (voice_time - days * 86400) / 3600;
                let minutes = (voice_time - days * 86400 - hours * 3600) / 60;
                let seconds = voice_time - days * 86400 - hours * 3600 - minutes * 60;

                let time_string = if days > 0 {
                    format!(
                        "**{}** days, **{}** hours, **{}** minutes, **{}** seconds",
                        days, hours, minutes, seconds
                    )
                } else if hours > 0 {
                    format!(
                        "**{}** hours, **{}** minutes, **{}** seconds",
                        hours, minutes, seconds
                    )
                } else if minutes > 0 {
                    format!("**{}** minutes, **{}** seconds", minutes, seconds)
                } else {
                    format!("**{}** seconds", seconds)
                }; 

                // calculate level difference
                let current_level = calculate_level(&member.xp);
                let new_level = calculate_level(&(member.xp + xp as u64));
                let level_difference = new_level - current_level;

                let requested_user = ctx.http.get_user(moved.user_id.0).await.unwrap().name.clone();

                // send message
                let _ = ChannelId(log_channel_id.clone().unwrap().parse::<u64>().unwrap())
                    .send_message(&ctx.http, |message| {
                        message.embed(|embed| {
                            embed.title(format!(
                                "{}'s voicetime",
                                requested_user
                                
                            ));
                            embed.description(time_string);

                            if level_difference > 0 {
                                embed.field(
                                    "Level",
                                    format!("**{} → {}**", crate::utils::utils::format_number(current_level as u64), crate::utils::utils::format_number(new_level as u64)),
                                    true,
                                );
                            }

                            embed.field("XP", crate::utils::utils::format_number(xp as u64), true);
                            embed.field("", "", true);
                            embed.color(colors::blue());
                            embed
                        })
                    })
                    .await;
            }

            // add xp to user
            member.xp += xp as u64;
            member = conform_xpc(member, &ctx, &guild_id.0, &moved.user_id.0).await;

            // update database
            let _ = GuildMember::set_xp(guild_id.0, moved.user_id.0, &member.xp, &member).await;

            // invalidate timestamp
            user.timestamps.join_voicechat = None;
            let _ = User::set(moved.user_id.0, user).await;
        }

        // check if moved from voicechat is the guilds afk channel or ignore channel
        if moved.channel_id.unwrap().0 == afk_channel_id.unwrap().0 || guild.clone().ignored.channels.unwrap().contains(&moved.channel_id.unwrap().0.to_string()) {
            let timestamp = chrono::Utc::now().timestamp() * 1000;
            let mut user = User::from_id(moved.user_id.0).await.unwrap();

            // check if voice module is enabled
            if !guild.modules.voicexp {
                return ();
            }
            
            // check if voice channel is ignored
            if guild.clone().ignored.channels.unwrap().contains(&moved.channel_id.unwrap().0.to_string()) {
                return ();
            }

            // set new timestamp
            user.timestamps.join_voicechat = Some(timestamp as u64);

            // update database
            let _ = User::set(moved.user_id.0, user).await;
        }
    }
}
