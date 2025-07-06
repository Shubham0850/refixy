use serde_json::{json, Value};
use reqwest::Client;

#[derive(Clone)]
pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // For testing - hardcode your API key here
        // let api_key = "xx".to_string();
        
        // Uncomment the line below and comment the line above to use environment variable instead
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not set")?;
        
        Ok(OpenAIClient {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn improve_text(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Refactor this text:\n\"{}\"",
            text
        );

        let request_body = json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 1000,
            "temperature": 0.7
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_json: Value = response.json().await?;
        
        let improved_text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to parse OpenAI response")?
            .trim()
            .to_string();

        Ok(improved_text)
    }
}
