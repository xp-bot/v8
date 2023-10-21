use serenity::{
    async_trait,
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};
use xp_db_connector::{guild::Guild, guild_member::GuildMember, user::User};

use crate::{
    commands::XpCommand,
    utils::{
        colors,
        math::{calculate_level, calculate_xp_from_voice_time},
        utils,
    },
};

pub struct VoicetimeCommand;

#[async_trait]
impl XpCommand for VoicetimeCommand {
    fn name(&self) -> &'static str {
        "voicetime"
    }

    fn register<'a>(
        &self,
        command: &'a mut serenity::builder::CreateApplicationCommand,
    ) -> &'a mut serenity::builder::CreateApplicationCommand {
        command
            .name(self.name())
            .description("Check how much time you have spent talking to friends.")
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to show the voicetime of.")
                    .kind(serenity::model::application::command::CommandOptionType::User)
                    .required(false)
            })
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut user_id = command.user.id.0;

        match command.data.options.first() {
            Some(option) => {
                if let Some(user) = Some(option.value.as_ref().unwrap().clone()) {
                    user_id = user.as_str().unwrap().parse::<u64>().unwrap();
                }
            }
            None => {}
        }

        let user = User::from_id(user_id).await?;

        let guild_member = GuildMember::from_id(command.guild_id.unwrap().0, user_id)
            .await
            .unwrap();

        let last_timestamp = user.timestamps.join_voicechat;

        let last_timestamp = match last_timestamp {
            Some(timestamp) => timestamp,
            None => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("This user has not joined a voice channel yet.");
                                message.ephemeral(true)
                            })
                    })
                    .await
                    .unwrap();

                return Ok(());
            }
        };

        // check if the user is currently in a voice channel
        let state = match ctx
            .cache
            .guild(command.guild_id.unwrap())
            .unwrap()
            .voice_states
            .get(&ctx.http.get_user(user_id).await?.id)
        {
            Some(state) => Some(state.clone()),
            None => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("This user is not currently in a voice channel.");
                                message.ephemeral(true)
                            })
                    })
                    .await
                    .unwrap();
                return Ok(());
            }
        };

        let guild = Guild::from_id(command.guild_id.unwrap().0).await?;

        // check if the user is in a voicechannel that's ignored
        if guild.clone().ignored.channels.unwrap().contains(&state.unwrap().channel_id.unwrap().0.to_string()) {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content("This voice channel is ignored.");
                            message.ephemeral(true)
                        })
                })
                .await
                .unwrap();
            return Ok(());
        }

        let current_timestamp = chrono::Utc::now().timestamp() * 1000;

        let boost_percentage = utils::calculate_total_boost_percentage(
            guild.clone(),
            &command.member.as_ref().unwrap().roles,
            ctx.cache
                .guild(command.guild_id.unwrap())
                .unwrap()
                .voice_states
                .get(&ctx.http.get_user(user_id).await?.id)
                .unwrap()
                .channel_id
                .unwrap()
                .0,
            ctx.cache
                .guild_channel(
                    ctx.cache
                        .guild(command.guild_id.unwrap())
                        .unwrap()
                        .voice_states
                        .get(&ctx.http.get_user(user_id).await?.id)
                        .unwrap()
                        .channel_id
                        .unwrap()
                        .0,
                )
                .unwrap()
                .parent_id,
        );

        let voice_xp = calculate_xp_from_voice_time(
            last_timestamp,
            current_timestamp,
            guild.values.voicexp,
            guild.values.voicejoincooldown,
            boost_percentage,
        );

        let current_level = calculate_level(&guild_member.xp);
        let new_level = calculate_level(&(guild_member.xp + voice_xp as u64));
        let level_difference = new_level - current_level;
        let voice_time = (current_timestamp - last_timestamp as i64) / 1000;

        let username = ctx.http.get_user(user_id).await?.name;

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

        let _ = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.title(format!("{}'s voicetime", username));
                            embed.description(time_string);

                            if level_difference > 0 {
                                embed.field(
                                    "Level",
                                    format!(
                                        "**{} → {}**",
                                        crate::utils::utils::format_number(current_level as i64),
                                        crate::utils::utils::format_number(new_level as i64)
                                    ),
                                    true,
                                );
                            }

                            embed.field(
                                "XP",
                                crate::utils::utils::format_number(voice_xp as i64),
                                true,
                            );
                            embed.field("", "", true);
                            embed.color(colors::blue());
                            embed
                        })
                    })
            })
            .await?;

        Ok(())
    }
}
