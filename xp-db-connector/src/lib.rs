pub mod user;
pub mod user_background;
pub mod guild_member;

use std::env;

pub type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_json<T>(url: String) -> Result<T, reqwest::Error>
where
    T: serde::de::DeserializeOwned,
{
    let client: reqwest::Client = reqwest::Client::new();

    let base_url: String = env::var("API_URL").expect("API_URL must be set");
    let api_auth: String = env::var("API_AUTH").expect("API_AUTH must be set");

    let response = client
        .get(format!("{}{}", base_url, url))
        .header("access", api_auth)
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(response)
}
