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

        if json["voted"].as_bool().unwrap() {
            return true;
        }
    }

    false
}
