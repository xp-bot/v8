use std::{env, time::Duration};

use events::handler::Handler;
use serenity::{client::bridge::gateway::GatewayIntents, Client};
use tokio::time::sleep;

mod events;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    // client initialization
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .intents(
            GatewayIntents::non_privileged()
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILDS,
        )
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
                    println!(
                        "Shard ID {} is {:?} ({:?})",
                        id.0 + 1,
                        runner.stage,
                        runner.latency.unwrap()
                    );
                } else {
                    println!("Shard ID {} is {:?}", id.0 + 1, runner.stage);
                }
            }
        }
    });

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
