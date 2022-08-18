use std::convert::Infallible;
use warp::{Rejection, http::StatusCode};
use handlebars::Handlebars;
use serde_json::{Value, Map};
use serde::Serialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref HOME: String = {
        let mut h = Handlebars::new();
        h.register_template_file("index", "templates/index.hbs").unwrap();

        let json: String = std::fs::read_to_string("content.json").unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        let map: &Map<String, Value> = parsed.as_object().unwrap();

        h.render("index", map).unwrap()
    };

    static ref E404: String = {
        let mut h = Handlebars::new();

        h.register_template_file("404", "templates/404.hbs").unwrap();

        let json: String = std::fs::read_to_string("content.json").unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        let map: &Map<String, Value> = parsed.as_object().unwrap();

        h.render("404", map).unwrap()
    };
}

/// GET /index
pub async fn index() -> Result<impl warp::Reply, Infallible> {    
    Ok(warp::reply::with_status(warp::reply::html(HOME.clone()), StatusCode::OK))
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        return Ok(warp::reply::with_status(warp::reply::html(E404.clone()), StatusCode::NOT_FOUND));
    } else if let Some(_e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = "BAD_REQUEST";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let error = ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    };

    let reply = warp::reply::html(
        serde_json::to_string(&error).unwrap()
    );

    Ok(warp::reply::with_status(reply, code))
}
