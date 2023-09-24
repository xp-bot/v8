pub mod guild;
pub mod guild_member;
pub mod guild_premium;
pub mod user;
pub mod user_background;

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
        .header("Authorization", format!("Bearer {}", api_auth))
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(response)
}

async fn post_json<T>(url: String, body: T) -> Result<(), reqwest::Error>
where
    T: serde::Serialize,
{
    let client: reqwest::Client = reqwest::Client::new();

    let base_url: String = env::var("API_URL").expect("API_URL must be set");
    let api_auth: String = env::var("API_AUTH").expect("API_AUTH must be set");

    let _ = client
        .post(format!("{}{}", base_url, url))
        .header("Authorization", format!("Bearer {}", api_auth))
        .json(&body)
        .send()
        .await?;

    Ok(())
}

async fn patch_json<T>(url: String, body: T) -> Result<(), reqwest::Error>
where
    T: serde::Serialize,
{
    let client: reqwest::Client = reqwest::Client::new();

    let base_url: String = env::var("API_URL").expect("API_URL must be set");
    let api_auth: String = env::var("API_AUTH").expect("API_AUTH must be set");

    let _ = client
        .patch(format!("{}{}", base_url, url))
        .header("Authorization", format!("Bearer {}", api_auth))
        .json(&body)
        .send()
        .await?;

    Ok(())
}

async fn delete_json(url: String) -> Result<(), reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();

    let base_url: String = env::var("API_URL").expect("API_URL must be set");
    let api_auth: String = env::var("API_AUTH").expect("API_AUTH must be set");

    let _ = client
        .delete(format!("{}{}", base_url, url))
        .header("Authorization", format!("Bearer {}", api_auth))
        .send()
        .await?;

    Ok(())
}
