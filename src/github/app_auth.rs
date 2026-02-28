#![allow(dead_code)]

use super::errors::{GitHubError, Result};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// GitHub App JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct AppJwtClaims {
    /// Issued at (current time)
    pub iat: i64,
    /// Expires at (10 minutes from now)
    pub exp: i64,
    /// GitHub App ID
    pub iss: u64,
}

/// Installation access token response from GitHub
#[derive(Debug, Deserialize)]
pub struct InstallationToken {
    pub token: String,
    pub expires_at: String,
    pub permissions: serde_json::Value,
    pub repositories: Option<Vec<serde_json::Value>>,
}

/// GitHub App authentication handler
pub struct GitHubAppAuth {
    app_id: u64,
    private_key: String,
}

impl GitHubAppAuth {
    /// Create a new GitHub App auth handler from private key file
    pub fn from_private_key_file<P: AsRef<Path>>(app_id: u64, key_path: P) -> Result<Self> {
        let private_key = fs::read_to_string(key_path)
            .map_err(|e| GitHubError::InvalidInput(format!("Failed to read private key: {}", e)))?;

        Ok(Self {
            app_id,
            private_key,
        })
    }

    /// Create a new GitHub App auth handler with inline private key
    pub fn new(app_id: u64, private_key: String) -> Self {
        Self {
            app_id,
            private_key,
        }
    }

    /// Generate JWT token for GitHub App authentication
    /// JWT is valid for 10 minutes (GitHub requirement)
    pub fn generate_jwt(&self) -> Result<String> {
        let now = Utc::now().timestamp();
        let exp = now + 600; // 10 minutes

        let claims = AppJwtClaims {
            iat: now,
            exp,
            iss: self.app_id,
        };

        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())
            .map_err(|e| GitHubError::InvalidInput(format!("Invalid private key: {}", e)))?;

        encode(
            &Header::new(jsonwebtoken::Algorithm::RS256),
            &claims,
            &encoding_key,
        )
        .map_err(|e| GitHubError::InvalidInput(format!("Failed to generate JWT: {}", e)))
    }

    /// Get installation access token for a specific installation
    pub async fn get_installation_token(
        &self,
        http_client: &reqwest::Client,
        installation_id: u64,
    ) -> Result<String> {
        let jwt = self.generate_jwt()?;

        let url = format!(
            "https://api.github.com/app/installations/{}/access_tokens",
            installation_id
        );

        let response = http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "ai-coder")
            .send()
            .await?;

        match response.status().as_u16() {
            200..=299 => {
                let token_response: InstallationToken = response.json().await?;
                Ok(token_response.token)
            }
            401 => Err(GitHubError::AuthenticationError),
            404 => Err(GitHubError::NotFound("Installation not found".to_string())),
            code => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(GitHubError::ApiError {
                    status: code,
                    message,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_claims_serialization() {
        let claims = AppJwtClaims {
            iat: 1234567890,
            exp: 1234568490,
            iss: 12345,
        };

        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("\"iat\":1234567890"));
        assert!(json.contains("\"iss\":12345"));
    }
}
