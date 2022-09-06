use warp::{Reply, reply, Rejection, reject, http::StatusCode, filters::body};
use std::{convert::Infallible, fs::read_to_string};
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    static ref INDEX_BODY: String = {
        read_to_string("templates/generated/index.html").unwrap()
    };
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_index() -> Result<impl Reply, Rejection> {
    return Ok(reply::with_status(reply::html(INDEX_BODY.clone()), StatusCode::OK));
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        let e404 = read_to_string("templates/generated/404.html").unwrap();
        return Ok(reply::with_status(reply::html(e404), StatusCode::NOT_FOUND));
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
