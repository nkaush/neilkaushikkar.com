use super::utils;

use serde::Deserialize;
use std::error::Error;
use reqwest::Client;

#[derive(Deserialize)]
pub struct AzureAccessToken {
    pub access_token: String,
    pub expires_in: u64,
    pub expires_on: u64,
    pub not_before: u64,
    pub resource: String,
    pub token_type: String,
}

impl AzureAccessToken {
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    pub async fn request_token(client: &Client, resource: &str) -> Result<AzureAccessToken, Box<dyn Error>> {
        let resp = client.get("http://169.254.169.254")
            .query(&[("api-version", "2018-02-01"), ("resource", resource)])
            .header("Metadata", "true")
            .send()
            .await?;

        Ok(resp.json::<AzureAccessToken>().await?)
    }
}