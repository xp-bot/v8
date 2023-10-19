#[derive(serde::Deserialize, Debug)]
pub struct OTDBResponse {
    pub response_code: u8,
    pub results: Vec<OTDBQuestion>,
}

#[derive(serde::Deserialize, Debug)]
pub struct OTDBQuestion {
    pub category: String,
    pub r#type: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

pub struct OpenTriviaDB;

impl OpenTriviaDB {
    pub async fn get_question() -> Result<OTDBResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://opentdb.com/api.php?amount=1&type=multiple")
            .send()
            .await?;
        let response_json: OTDBResponse = response.json().await?;
        Ok(response_json)
    }
}
