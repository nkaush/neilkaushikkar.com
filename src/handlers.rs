use std::convert::Infallible;
use warp::http::StatusCode;
use handlebars::Handlebars;
use serde_json::{Value, Map};
use lazy_static::lazy_static;

lazy_static! {
    static ref BODY: String = {
        let mut h = Handlebars::new();
        h.register_template_file("index-template", "templates/index.hbs").unwrap();
        // h.register_template_file("skills", "templates/sections/skills.hbs").unwrap();
        // h.register_template_file("timeline", "templates/sections/timeline.hbs").unwrap();
        // h.register_template_file("portfolio", "templates/sections/portfolio.hbs").unwrap();
        // h.register_template_file("affiliations", "templates/sections/affiliations.hbs").unwrap();
        // h.register_template_file("portfolio_modals", "templates/sections/portfolio_modals.htmls").unwrap();

        let json: String = std::fs::read_to_string("content.json").unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        let map: &Map<String, Value> = parsed.as_object().unwrap();

        h.render("index-template", map).unwrap()
    };
}

/// GET /index
pub async fn index() -> Result<impl warp::Reply, Infallible> {    
    Ok(warp::reply::with_status(warp::reply::html(BODY.clone()), StatusCode::OK))
}
