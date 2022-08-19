use warp::{Reply, reply, Rejection, reject, http::StatusCode, filters::body};
use std::convert::Infallible;
use serde_json::{Value, Map};
use handlebars::Handlebars;
use serde::Serialize;

lazy_static::lazy_static! {
    static ref DESERIALIZED_DATA: Value = {
        let json: String = std::fs::read_to_string("content.json").unwrap();
        serde_json::from_str(&json).unwrap()
    };

    static ref HOME: String = {
        let mut h = Handlebars::new();
        h.register_template_file("index", "templates/index.hbs").unwrap();
        h.register_template_file("base", "templates/base.hbs").unwrap();
        let map: &Map<String, Value> = DESERIALIZED_DATA.as_object().unwrap();

        h.render("index", map).unwrap()
    };

    static ref E404: String = {
        let mut h = Handlebars::new();
        h.register_template_file("404", "templates/404.hbs").unwrap();
        h.register_template_file("base", "templates/base.hbs").unwrap();
        let map: &Map<String, Value> = DESERIALIZED_DATA.as_object().unwrap();

        h.render("404", map).unwrap()
    };
}

/// GET /index
pub async fn index() -> Result<impl Reply, Infallible> {    
    Ok(reply::with_status(reply::html(HOME.clone()), StatusCode::OK))
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        return Ok(reply::with_status(reply::html(E404.clone()), StatusCode::NOT_FOUND));
    } else if let Some(_e) = err.find::<body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = "BAD_REQUEST";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
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

    let reply = reply::html(
        serde_json::to_string(&error).unwrap()
    );

    Ok(reply::with_status(reply, code))
}
