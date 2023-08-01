use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::{Activity, GuildId, Interaction, Ready},
    prelude::{Context, EventHandler},
};

use crate::commands;

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
                commands
                    .create_application_command(|command| {
                        commands::misc::leaderboard::register(command)
                    })
                    .create_application_command(|command| commands::misc::about::register(command))
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
            match command.data.name.as_str() {
                "leaderboard" => commands::misc::leaderboard::exec(ctx, command).await,
                "about" => commands::misc::about::exec(ctx, command).await,
                _ => {
                    error!("Received unknown command: {:?}", command);
                    return ();
                }
            };
        }
    }
}
