use events::handler::Handler;
use log::{error, info};
use serenity::{prelude::GatewayIntents, Client, client::bridge::gateway::ShardId};
use std::{env, time::Duration, collections::HashMap};
use tokio::time::sleep;

use crate::utils::{topgg::post_bot_stats, ilum::send_shard_report};

mod commands;
mod events;
mod utils;

#[tokio::main]
async fn main() {
    let _ = match dotenv::dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(_) => {
            println!("No .env file found, using env vars");
        }
    };

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // client initialization
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(
        &token,
        GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILDS
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .expect("Err creating client");

    // sharding
    let manager = client.shard_manager.clone();
    let cache = client.cache_and_http.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            let guilds = cache.cache.guilds();
            let mut shard_guild_count: HashMap<ShardId, usize> = HashMap::new();
            for guild in guilds {
                let guild_id: u64 = guild.0;
                let shard_id: u64 = serenity::utils::shard_id(guild_id, shard_runners.len() as u64);

                match shard_guild_count.get(&ShardId(shard_id)) {
                    Some(count) => shard_guild_count.insert(ShardId(shard_id), count + 1),
                    None => shard_guild_count.insert(ShardId(shard_id), 1),
                };
            }

            for (id, runner) in shard_runners.iter() {
                if runner.latency.is_some() {
                    
                    post_bot_stats(id.0, shard_guild_count.get(&id).unwrap().to_owned(), shard_runners.len() as u64).await;

                    let latency = runner.latency.unwrap().as_millis() as i64;
                    let connected = runner.stage == serenity::gateway::ConnectionStage::Connected;
                    send_shard_report(id.0, shard_guild_count.get(&id).unwrap().to_owned(), latency, connected).await;

                    info!(
                        "Shard {} is {:?} ({:?} latency)",
                        id.0 + 1,
                        runner.stage,
                        runner.latency.unwrap()
                    );
                } else {
                    info!("Shard {} is {:?}", id.0 + 1, runner.stage);
                }
            }
        }
    });

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
