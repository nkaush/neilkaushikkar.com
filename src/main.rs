mod filters;
mod handlers;

use std::{io, env, fs::File, error::Error};
use std::net::Ipv4Addr;
use futures::future;
use warp::Filter;
use reqwest;

async fn download_file(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await.expect("body invalid");
    let mut out = File::create(path)?;
    io::copy(&mut body.as_bytes(), &mut out)?;
    Ok(path.into())
}

async fn load_secrets() -> Result<(String, String), Box<dyn Error>> {
    let env_private_key_uri = "PRIVATE_KEY_LOOKUP";
    let env_cert_uri = "CERT_LOOKUP";

    let private_key_uri = env::var(env_private_key_uri)?;
    let cert_uri = env::var(env_cert_uri)?;

    let (key_path, cert_path) = future::join(
        download_file(&private_key_uri, "private.key"),
        download_file(&cert_uri, "fullchain.pem")
    ).await;

    return Ok((key_path?, cert_path?));
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
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=index=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "index=info");
    }
    pretty_env_logger::init();

    let api = filters::all_routes().recover(handlers::handle_rejection);

    // View access logs by setting `RUST_LOG=index`.
    let routes = api.with(warp::log("index"));

    // Start up the server...
    let http_port_key = "PORT";

    let port: u16 = match env::var(http_port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    match load_secrets().await {
        Ok((key_path, cert_path)) => {
            let (_http_addr, http_warp) = warp::serve(routes.clone())
                .bind_ephemeral((Ipv4Addr::UNSPECIFIED, port));

            let (_https_addr, https_warp) = warp::serve(routes)
                .tls()
                .cert_path(cert_path)
                .key_path(key_path)
                .bind_ephemeral((Ipv4Addr::UNSPECIFIED, 443));

            println!("Starting HTTP & HTTPS server with TLS");
            future::join(http_warp, https_warp).await;
        },
        Err(e) => {
            println!("Error loading secrets: {}", e);
            println!("Starting HTTP server without TLS");
            warp::serve(routes)
                .run(([0, 0, 0, 0], port))
                .await;
        }
    }
}
