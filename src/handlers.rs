use std::convert::Infallible;
use warp::http::StatusCode;
use handlebars::Handlebars;
use serde_json::{Value, Map};

/// GET /index
pub async fn index() -> Result<impl warp::Reply, Infallible> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index-template", "templates/index.hbs").unwrap();

    // register partials
    handlebars.register_template_file("skills", "templates/sections/skills.hbs").unwrap();
    handlebars.register_template_file("timeline", "templates/sections/timeline.hbs").unwrap();
    handlebars.register_template_file("portfolio", "templates/sections/portfolio.hbs").unwrap();
    handlebars.register_template_file("affiliations", "templates/sections/affiliations.hbs").unwrap();

    // handlebars.register_template_file("portfolio_modals", "templates/sections/portfolio_modals.htmls").unwrap();

    let config_data = std::fs::read_to_string("content.json").unwrap();

    let parsed: Value = serde_json::from_str(&config_data).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    let body: String = handlebars.render("index-template", &obj).unwrap();
    
    Ok(warp::reply::with_status(warp::reply::html(body), StatusCode::OK))
}
