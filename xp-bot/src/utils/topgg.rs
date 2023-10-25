use serde_json::json;

pub async fn check_user_vote(user_id: &u64) -> bool {
    let client = reqwest::Client::new();

    let topgg_token = dotenv::var("TOPGG_TOKEN").expect("Expected a token in the environment");

    let response = client
        .get(format!(
            "https://top.gg/api/bots/{}/check?userId={}",
            "706935674800177193", user_id
        ))
        .header("Authorization", topgg_token)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();

        if json["voted"].as_i64() == Some(1) {
            return true;
        }
    }

    false
}

pub async fn post_bot_stats(shard_id: u64, guild_count: usize, shard_count: u64) {
    let client = reqwest::Client::new();
    let topgg_token = dotenv::var("TOPGG_TOKEN").expect("Expected a token in the environment");

    let body = json!({
        "shard_id": shard_id,
        "shard_count": shard_count,
        "server_count": guild_count
    });

    let _ = client
        .post(format!(
            "https://top.gg/api/bots/{}/stats",
            "706935674800177193"
        ))
        .header("Authorization", topgg_token)
        .json(&body)
        .send()
        .await
        .unwrap();
}
