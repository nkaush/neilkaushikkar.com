use warp::{Reply, reply, Rejection, reject, http::StatusCode, filters::body};
use std::{convert::Infallible, fs::read_to_string};
use lazy_static::lazy_static;

lazy_static! {
    static ref INDEX_BODY: String = {
        read_to_string("templates/generated/index.html").unwrap()
    };
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

    let json = format!("{{\"code\":{},\"message\":\"{}\"}}", code.as_u16(), message);
    let reply = reply::html(json);

    Ok(reply::with_status(reply, code))
}
