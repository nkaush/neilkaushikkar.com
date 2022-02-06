mod helpers;
mod filters;
mod handlers;

use std::net::Ipv4Addr;
use std::env;
use warp::Filter;


/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=index=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "index=info");
    }
    pretty_env_logger::init();


    let api = filters::all_routes();

    // View access logs by setting `RUST_LOG=index`.
    let routes = api.with(warp::log("index"));
    // Start up the server...
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";

    // let port_key = "PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}
