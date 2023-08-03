use events::handler::Handler;
use log::{error, info};
use serenity::{Client, prelude::GatewayIntents};
use std::{env, time::Duration};
use tokio::time::sleep;

mod commands;
mod events;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // client initialization
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token, 
        GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILDS,
    )
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // sharding
    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                if runner.latency.is_some() {
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
