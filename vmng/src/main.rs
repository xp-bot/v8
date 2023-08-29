use std::env;
use serde::{Serialize, Deserialize};

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

    println!("Approximate guild count: {}", application.approximate_guild_count);

    // deploy ghcr packages raeys-v8 and raeys-v8-beta
}
