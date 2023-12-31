use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};
use xp_db_connector::{guild::Guild, guild_premium::GuildPremium};

use crate::{commands::XpCommand, utils::colors};

pub struct SettingsCommand;

#[async_trait]
impl XpCommand for SettingsCommand {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("settings")
            .description("View your servers settings via one simple command!")
            .create_option(|option| {
                option
                    .name("setting")
                    .description("The setting you want to view.")
                    .kind(serenity::model::application::command::CommandOptionType::String)
                    .required(true)
                    .add_string_choice("Modules", "modules")
                    .add_string_choice("Values", "values")
                    .add_string_choice("Roles", "roles")
                    .add_string_choice("Boosts", "boosts")
                    .add_string_choice("Ignores", "ignores")
            })
    }

    async fn exec(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let guild_id = command.guild_id.unwrap();
        let option = command.data.options[0].clone();

        let mut guild = match Guild::from_id(guild_id.into()).await {
            Ok(guild) => guild,
            Err(why) => {
                log::error!("Could not get guild: {:?}", why);
                return Ok(());
            }
        };
        let guild_premium = GuildPremium::from_id(guild_id.into()).await?;

        let mut fields: Vec<(String, String, bool)> = Vec::new();

        let mut option_value = String::new();
        match option.value.unwrap().as_str().unwrap() {
            "modules" => {
                option_value = "Modules".to_string();
                fields.push((
                    format!("{} Message xp", tick_helper(guild.modules.messagexp)),
                    "Let users receive xp by writing messages.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Voice xp", tick_helper(guild.modules.voicexp)),
                    "Let users receive xp by spending time in a voice channel.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Reaction xp", tick_helper(guild.modules.reactionxp)),
                    "Let users receive xp by reacting to messages.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Ignore AFK", tick_helper(guild.modules.ignoreafk)),
                    "Disable xp in AFK Channels.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Autonick", tick_helper(guild.modules.autonick)),
                    "Automatically show the level of each user in their nicknames.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "{} Autonick Use Prefix",
                        tick_helper(guild.modules.autonickuseprefix)
                    ),
                    format!(
                        "`{}` Show the level on the left side of a user's nickname.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!(
                        "{} Autonick Show String",
                        tick_helper(guild.modules.autonickshowstring)
                    ),
                    format!(
                        "`{}` Show \"Lvl.\" in the nickname when Autonick is enabled.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!("{} Leaderboard", tick_helper(guild.modules.leaderboard)),
                    "Enable ranking in your community".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Single Rank Role", tick_helper(guild.modules.singlerankrole)),
                    "Always give your users only the highest achieved role, and remove all level roles below it.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Remove Reached Levelroles", tick_helper(guild.modules.removereachedlevelroles)),
                    format!(
                        "`{}` Remove roles that are above a user's level as soon as the user's xp has been reduced.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!("{} Maximum Level", tick_helper(guild.modules.maximumlevel)),
                    "Limit the maximum level that users can reach.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "{} Reset User On Leave",
                        tick_helper(guild.modules.resetonleave)
                    ),
                    format!(
                        "`{}` Delete the user data as soon as a user leaves the server.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!(
                        "{} Enable Commands In Threads",
                        tick_helper(guild.modules.enablecommandsinthreads)
                    ),
                    format!(
                        "`{}` Let users use xp commands in threads.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!("{} Games", tick_helper(guild.modules.games)),
                    "Let your users earn xp by playing games.".to_string(),
                    true,
                ));
                fields.push((
                    format!("{} Trivia", tick_helper(guild.modules.trivia)),
                    "Let your users prove their knowledge.".to_string(),
                    true,
                ));
            }
            "values" => {
                option_value = "Values".to_string();
                fields.push((
                    format!(
                        "Message xp: {}",
                        crate::utils::utils::format_number(guild.values.messagexp as i64)
                    ),
                    "The amount of xp a user gets per message.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Message cooldown: {}",
                        crate::utils::utils::format_number(guild.values.messagecooldown as i64)
                    ),
                    "The time in seconds until the next message will be counted.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Voice xp: {}",
                        crate::utils::utils::format_number(guild.values.voicexp as i64)
                    ),
                    "The amount of xp a user receives per minute in the voicechat.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Voice Join Cooldown: {}",
                        crate::utils::utils::format_number(guild.values.voicejoincooldown as i64)
                    ),
                    "The time in seconds until XP starts measuring the user's voicechat time."
                        .to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Reaction xp: {}",
                        crate::utils::utils::format_number(guild.values.reactionxp as i64)
                    ),
                    "The amount of xp a user gets per reaction.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Loot xp: {}",
                        crate::utils::utils::format_number(guild.values.lootXP as i64)
                    ),
                    "The amount of xp a user gets for playing `/loot`.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Fish xp: {}",
                        crate::utils::utils::format_number(guild.values.fishXP as i64)
                    ),
                    "The amount of xp a user gets for playing `/fish`.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Roll xp: {}",
                        crate::utils::utils::format_number(guild.values.rollXP as i64)
                    ),
                    "The amount of xp a user gets for playing `/roll`.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Game cooldown: {}",
                        crate::utils::utils::format_number(guild.values.gamecooldown as i64)
                    ),
                    format!(
                        "`{}` The time in seconds until a user start another game.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!(
                        "Trivia xp: {}",
                        crate::utils::utils::format_number(guild.values.triviaxp as i64)
                    ),
                    "The maximum amount of xp a user gets for playing trivia.".to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Trivia cooldown: {}",
                        crate::utils::utils::format_number(guild.values.triviacooldown as i64)
                    ),
                    format!(
                        "`{}` The time in seconds until a user can start a new Trivia game.",
                        premium_helper(guild_premium.premium)
                    ),
                    true,
                ));
                fields.push((
                    format!(
                        "Maximum daily xp: {}",
                        crate::utils::utils::format_number(guild.values.maximumdailyxp as i64)
                    ),
                    "The maximum amount of xp obtainable by executing the `/daily` command."
                        .to_string(),
                    true,
                ));
                fields.push((
                    format!(
                        "Maximum level: {}",
                        crate::utils::utils::format_number(guild.values.maximumlevel as i64)
                    ),
                    "The maximum level that can be reached.".to_string(),
                    true,
                ));
            }
            "roles" => {
                option_value = "Roles".to_string();
                let mut autorole = "There is currently no autorole set.".to_string();
                let mut levelroles = "There are currently no levelroles set.".to_string();

                // sort guild.levelroles by level
                guild.levelroles.sort_by(|a, b| a.level.cmp(&b.level));

                let mut levelroles_string = String::new();
                guild.levelroles.iter().for_each(|levelrole| {
                    if levelrole.level == -1 {
                        autorole = format!("<@&{}>", levelrole.id);
                    } else {
                        levelroles_string.push_str(&format!(
                            "<@&{}> at level {}\n",
                            levelrole.id, levelrole.level
                        ));
                        levelroles = levelroles_string.to_owned();
                    }
                });

                fields.push(("Autorole".to_string(), autorole, false));
                fields.push(("Levelroles".to_string(), levelroles, false));
            }
            "boosts" => {
                option_value = "Boosts".to_string();
                let mut boostroles = "There are currently no boosted roles set.".to_string();
                let mut boostchannels = "There are currently no boosted channels set.".to_string();
                let mut boostcategories =
                    "There are currently no boosted categories set.".to_string();

                let mut boostroles_string = String::new();
                guild.boosts.roles.iter().for_each(|boostrole| {
                    boostroles_string.push_str(&format!(
                        "<@&{}> with {}% boost\n",
                        boostrole.id, boostrole.percentage
                    ));
                    boostroles = boostroles_string.to_owned();
                });

                let mut boostchannels_string = String::new();
                guild.boosts.channels.iter().for_each(|boostchannel| {
                    boostchannels_string.push_str(&format!(
                        "<#{}> with {}% boost\n",
                        boostchannel.id, boostchannel.percentage
                    ));
                    boostchannels = boostchannels_string.to_owned();
                });

                let mut boostcategories_string = String::new();
                if guild.boosts.categories.is_some() {
                    guild
                        .boosts
                        .categories
                        .as_ref()
                        .unwrap()
                        .iter()
                        .for_each(|boostcategory| {
                            boostcategories_string.push_str(&format!(
                                "<#{}> with {}% boost\n",
                                boostcategory.id, boostcategory.percentage
                            ));
                            boostcategories = boostcategories_string.to_owned();
                        });
                }

                fields.push(("Boosted roles".to_string(), boostroles, false));
                fields.push(("Boosted channels".to_string(), boostchannels, false));
                fields.push(("Boosted categories".to_string(), boostcategories, false));
            }
            "ignores" => {
                option_value = "Ignores".to_string();
                let mut ignoredroles = "There are currently no ignored roles set.".to_string();
                let mut ignoredchannels =
                    "There are currently no ignored channels set.".to_string();
                let mut ignoredcategories =
                    "There are currently no ignored categories set.".to_string();

                let mut ignoredroles_string = String::new();
                guild.ignored.roles.iter().for_each(|ignoredrole| {
                    ignoredroles_string.push_str(&format!("<@&{}>\n", ignoredrole));
                    ignoredroles = ignoredroles_string.to_owned();
                });

                let mut ignoredchannels_string = String::new();
                if guild.ignored.channels.is_some() {
                    guild
                        .ignored
                        .channels
                        .as_ref()
                        .unwrap()
                        .iter()
                        .for_each(|ignoredchannel| {
                            ignoredchannels_string.push_str(&format!("<#{}>\n", ignoredchannel));
                            ignoredchannels = ignoredchannels_string.to_owned();
                        });
                }

                let mut ignoredcategories_string = String::new();
                if guild.ignored.categories.is_some() {
                    guild
                        .ignored
                        .categories
                        .as_ref()
                        .unwrap()
                        .iter()
                        .for_each(|ignoredcategory| {
                            ignoredcategories_string.push_str(&format!("<#{}>\n", ignoredcategory));
                            ignoredcategories = ignoredcategories_string.to_owned();
                        });
                }

                fields.push(("Ignored roles".to_string(), ignoredroles, false));
                fields.push(("Ignored channels".to_string(), ignoredchannels, false));
                fields.push(("Ignored categories".to_string(), ignoredcategories, false));
            }
            _ => {}
        }
        let result = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed: &mut CreateEmbed| {
                            embed
                                .title(format!("{} settings", option_value))
                                .description(format!(
                                    "Here are all {} settings for this community.",
                                    option_value.to_ascii_lowercase()
                                ))
                                .colour(colors::blue())
                                .fields(fields)
                                .footer(|footer| footer.text("🔒 Premium feature"))
                        })
                    })
            })
            .await;

        if let Err(why) = result {
            log::error!("Could not respond to command: {:?}", why);
        }

        // check if user has manage_server permission
        if !command
            .member
            .as_ref()
            .unwrap()
            .permissions
            .unwrap()
            .manage_guild()
        {
            return Ok(());
        }

        let _ = command
            .create_followup_message(&ctx.http, |message| {
                message
                    .content(format!("You can edit these settings in the dashboard: https://xp-bot.net/servers/{}/{}", guild_id, option_value.to_ascii_lowercase()))
                    .ephemeral(true)
            })
            .await?;

        Ok(())
    }
}

fn tick_helper(enabled: bool) -> String {
    if enabled {
        "<:tickYes:792577321109422111>".to_string()
    } else {
        "<:tickNo:792577321068527666>".to_string()
    }
}

fn premium_helper(enabled: bool) -> String {
    if enabled {
        "🔓".to_string()
    } else {
        "🔒".to_string()
    }
}
