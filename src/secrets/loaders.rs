use super::{utils, models::AzureAccessToken};

use std::{env, error::Error};
use futures::future;
use reqwest::Client;

static ENV_CERT_LOOKUP: &str = "CERT_LOOKUP";
static ENV_PRIVATE_KEY_LOOKUP: &str = "PRIVATE_KEY_LOOKUP";

static BLOB_CERTIFICATE: &str = "https://storageaccountrgnei86c8.blob.core.windows.net/secrets/fullchain.pem";
static BLOB_PRIVATE_KEY: &str = "https://storageaccountrgnei86c8.blob.core.windows.net/secrets/privkey.pem";
static BLOB_RESOURCE_TYPE: &str = "https://storage.azure.com";

struct PrivateKeyLoader;

impl PrivateKeyLoader {

    fn try_role_assignment(path: &str, client: &Client) -> Result<String, Box<dyn Error>> {
        AzureRoleAssignmentSecretLoader::load_url_to_file(BLOB_PRIVATE_KEY, path, BLOB_RESOURCE_TYPE, client)
    }

    fn try_shared_access_signature

    pub async fn load_to_file(path: &str, client: &Client) -> Result<String, Box<dyn Error>> {
        AzureRoleAssignmentSecretLoader::load_url_to_file(
                BLOB_PRIVATE_KEY, path, BLOB_RESOURCE_TYPE, client)
            .await
            .or_else(|_| Ok("".into()))
    }
}

struct AzureSharedAccessSignatureSecretLoader;

impl AzureSharedAccessSignatureSecretLoader {
    pub async fn load_url_to_file(env_var: &str, path: &str, client: &Client) -> Result<String, Box<dyn Error>> {
        let url = env::var(env_var)?;
        utils::download_file(client, &url, path, None).await
    }
}

struct AzureRoleAssignmentSecretLoader;

impl AzureRoleAssignmentSecretLoader {
    pub async fn load_url_to_file(url: &str, path: &str, resource: &str, client: &Client) -> Result<String, Box<dyn Error>> {
        let token = AzureAccessToken::request_token(client, resource).await?;
        utils::download_file(client, url, path, Some(token.access_token())).await
    }
}

pub async fn load_secrets() -> Result<(String, String), Box<dyn Error>> {
    let cert_uri = env::var(ENV_CERT_LOOKUP)?;
    let private_key_uri = env::var(ENV_PRIVATE_KEY_LOOKUP)?;

    let (cert_path, key_path) = future::join(
        utils::download_file(&cert_uri, "fullchain.pem"),
        utils::download_file(&private_key_uri, "privkey.pem")
    ).await;

    return Ok((cert_path?, key_path?));
}