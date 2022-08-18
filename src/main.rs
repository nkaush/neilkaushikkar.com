mod filters;
mod handlers;

use std::net::Ipv4Addr;
use std::env;
use warp::Filter;

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
    let port_key = "PORT";

    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
