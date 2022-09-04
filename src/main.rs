use std::{io, env, fs::File, error::Error, net::Ipv4Addr};
use webserver::{filters, handlers};
use log::{info, debug};
use futures::future;
use warp::Filter;
use reqwest;

static DEFAULT_HTTP_PORT: u16 = 8080;
static DEFAULT_HTTPS_PORT: u16 = 8443;

static ENV_LOG_FILTER: &str = "RUST_LOG";
static ENV_HTTP_PORT: &str = "HTTP_PORT";
static ENV_HTTPS_PORT: &str = "HTTPS_PORT";
static ENV_CERT_LOOKUP: &str = "CERT_LOOKUP";
static ENV_PRIVATE_KEY_LOOKUP: &str = "PRIVATE_KEY_LOOKUP";

async fn download_file(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    debug!("Downloading {} to {}.", url, path);
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    let mut out = File::create(path)?;
    io::copy(&mut body.as_bytes(), &mut out)?;
    debug!("Download to {} finished.", path);
    Ok(path.into())
}

async fn load_secrets() -> Result<(String, String), Box<dyn Error>> {
    let cert_uri = env::var(ENV_CERT_LOOKUP)?;
    let private_key_uri = env::var(ENV_PRIVATE_KEY_LOOKUP)?;

    let (cert_path, key_path) = future::join(
        download_file(&cert_uri, "fullchain.pem"),
        download_file(&private_key_uri, "privkey.pem")
    ).await;

    return Ok((cert_path?, key_path?));
}

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /`: return the homepage as HTML
/// - `GET /resume`: return my resume as a PDF
/// - `GET /static/<path>`: serve any static files stored in the project
#[tokio::main]
async fn main() {
    if env::var_os(ENV_LOG_FILTER).is_none() {
        // Set `RUST_LOG=debug` to see debug logs,
        // this only shows access logs.
        env::set_var(ENV_LOG_FILTER, "info");
    }
    pretty_env_logger::init();

    let api = filters::all_routes().recover(handlers::handle_rejection);

    // View access logs by setting `RUST_LOG=index`.
    let routes = api.with(warp::log("index"));

    // Start up the server...
    let http_port: u16 = match env::var(ENV_HTTP_PORT) {
        Ok(val) => val.parse().expect("HTTP_PORT is not a number!"),
        Err(_) => DEFAULT_HTTP_PORT,
    };

    let https_port: u16 = match env::var(ENV_HTTPS_PORT) {
        Ok(val) => val.parse().expect("HTTPS_PORT is not a number!"),
        Err(_) => DEFAULT_HTTPS_PORT,
    };

    match load_secrets().await {
        Ok((cert_path, key_path)) => {
            let (_http_addr, http_warp) = warp::serve(routes.clone())  
                .bind_ephemeral((Ipv4Addr::UNSPECIFIED, http_port));

            let (_https_addr, https_warp) = warp::serve(routes)
                .tls()
                .cert_path(&cert_path)
                .key_path(&key_path)
                .bind_ephemeral((Ipv4Addr::UNSPECIFIED, https_port));

            info!("Starting server with TLS.");
            info!("Listening on ports {} and {}...", http_port, https_port);
            future::join(http_warp, https_warp).await;
        },
        Err(e) => {
            info!("Error loading secrets: {}", e);
            info!("Starting server without TLS.");
            info!("Listening on port {}...", http_port);
            warp::serve(routes)
                .run((Ipv4Addr::UNSPECIFIED, http_port))
                .await;
        }
    }
}