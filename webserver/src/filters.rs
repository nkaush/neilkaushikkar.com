use warp::{Filter, Reply, Rejection, fs, compression};
use super::handlers::handle_index;

/// combine all routes
pub fn all_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index()
        .or(home())
        .or(serve_static())
        .or(resume())
        .or(favicon())
        .or(robots_txt())
        .or(webroot_acme_challenge())
}

fn home() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handle_index)
        .with(compression::gzip())
}

fn index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("index")
        .and(warp::get())
        .and_then(handle_index)
        .with(compression::gzip())
}

fn serve_static() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("static")
        .and(warp::get())
        .and(fs::dir("www/static"))
}

fn resume() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("resume")
        .and(warp::get())
        .and(fs::file("www/static/doc/resume.pdf"))
}

fn favicon() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("favicon.ico")
        .and(warp::get())
        .and(fs::file("www/static/images/favicon.png"))
}

fn robots_txt() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("robots.txt")
        .and(warp::get())
        .and(fs::file("www/robots.txt"))
}

fn webroot_acme_challenge() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(".well-known")
        .and(warp::path("acme-challenge"))
        .and(warp::get())
        .and(fs::dir(".well-known/acme-challenge"))
}