use reqwest::Response;

pub async fn get_authenticated(url: String) -> Response {
    let auth_key = dotenv::var("API_AUTH").expect("API_AUTH must be set");
    let base_url = dotenv::var("API_URL").expect("API_URL must be set");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}{}", base_url, url))
        .header("access", auth_key)
        .send()
        .await
        .unwrap();

    response
}
