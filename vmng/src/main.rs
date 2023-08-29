use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Application {
    approximate_guild_count: u64,
}

fn main() {
    let _ = match dotenv::dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(_) => {
            println!("No .env file found, using env vars");
        }
    };

    let url = "https://discord.com/api/applications/@me";
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .unwrap();

    let application: Application = response.json().unwrap();
    println!(
        "Approximate guild count: {}",
        application.approximate_guild_count
    );

    let shard_count = application.approximate_guild_count / 1000 + 1;
    println!("Shard count: {}", shard_count);

    // 25% of the shards are used for preview
    let preview_shard_count = shard_count / 4;
    let prod_shard_count = shard_count - preview_shard_count;
    println!("Preview shard count: {}", preview_shard_count);
    println!("Production shard count: {}", prod_shard_count);

    // stop and remove old containers
    std::process::Command::new("docker")
        .args(&["stop", "xpbot-v8"])
        .status()
        .expect("failed to execute process");

    std::process::Command::new("docker")
        .args(&["rm", "xpbot-v8"])
        .status()
        .expect("failed to execute process");

    std::process::Command::new("docker")
        .args(&["stop", "xpbot-v8-preview"])
        .status()
        .expect("failed to execute process");

    std::process::Command::new("docker")
        .args(&["rm", "xpbot-v8-preview"])
        .status()
        .expect("failed to execute process");

    // deploy ghcr packages raeys-v8 and raeys-v8-preview with docker
    std::process::Command::new("docker")
        .args(&["run", "-d", "--env .env", "--name xpbot-v8", "ghcr.io/xp-bot/raeys-v8:latest"])
        .status()
        .expect("failed to execute process");

    std::process::Command::new("docker")
        .args(&["run", "-d", "--env .env", "--name xpbot-v8-preview",  "ghcr.io/xp-bot/raeys-v8-preview:latest"])
        .status()
        .expect("failed to execute process");
}
