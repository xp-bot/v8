use serde_json::json;

pub async fn send_shard_report(shard_id: u64, server_count: usize, latency: i64, gateway_connected: bool) {
    let client = reqwest::Client::new();

    let ilum_auth = std::env::var("ILUM_AUTH").expect("Expected a token in the environment");

    let body = json!({
        "shard_id": shard_id,
        "server_count": server_count,
        "websocket_ping": latency,
        "gateway_connected": gateway_connected
    });

    log::debug!("Sending shard report: {}", body.to_string());

    let res = client
        .post("https://ilumv2.xp-bot.net/submit/shard")
        .header("Content-Type", "application/json")
        .header("Authorization", ilum_auth)
        .body(body.to_string())
        .send()
        .await
        .unwrap();
    log::debug!("Shard report sent: {}", res.status());
}