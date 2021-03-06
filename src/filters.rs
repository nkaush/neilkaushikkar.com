use super::handlers;
use warp::Filter;

/// combine all routes
pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    index()
        .or(home())
        .or(serve_static())
        .or(resume())
        .or(favicon())
}

pub fn home() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::index)
}

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("index")
        .and(warp::get())
        .and_then(handlers::index)
}

pub fn serve_static() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("www/static"))
}

pub fn resume() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("resume").and(warp::fs::file("www/static/doc/resume.pdf"))
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("favicon.ico").and(warp::fs::file("www/static/images/favicon.png"))
}