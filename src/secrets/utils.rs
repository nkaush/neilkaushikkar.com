use std::{io, fs::File, error::Error};
use reqwest::Client;

use log::debug;
use reqwest;

pub fn get_client() -> Result<Client, Box<dyn Error>> {
    Ok(Client::builder().build()?)
}

pub async fn download_file(client: &Client, url: &str, path: &str, bearer_auth: Option<&str>) -> Result<String, Box<dyn Error>> {
    debug!("Downloading {} to {}.", url, path);

    let resp = match bearer_auth {
        Some(token) => client.get(url).bearer_auth(token).send().await?,
        None => client.get(url).send().await?
    };

    let body = resp.text().await?;

    let mut out = File::create(path)?;
    io::copy(&mut body.as_bytes(), &mut out)?;
    debug!("Download to {} finished.", path);

    Ok(path.into())
}
