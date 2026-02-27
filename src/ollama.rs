use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}
