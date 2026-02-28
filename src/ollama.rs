use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    #[serde(default)]
    pub response: String,
    #[serde(default)]
    pub done: bool,
    pub error: Option<String>,
}
