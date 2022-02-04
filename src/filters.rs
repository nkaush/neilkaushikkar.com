use super::handlers;
use warp::Filter;

/// combine all routes
pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    index()
        .or(serve_static())
        .or(resume())
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
