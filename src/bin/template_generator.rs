use webserver::content;
use std::error::Error;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    info!("Loading data...");
    let mut content = content::Content::init("content.json");

    info!("Loading templates...");
    content.register_template("base", "templates/base.hbs")?;
    content.register_template("index", "templates/index.hbs")?;
    content.register_template("404", "templates/404.hbs")?;

    content.register_template("al22_modal", "templates/modals/al22.hbs")?;
    content.register_template("cs128h_modal", "templates/modals/cs128h.hbs")?;
    content.register_template("twilio_modal", "templates/modals/twilio.hbs")?;
    content.register_template("burd_modal", "templates/modals/burd.hbs")?;
    content.register_template("luis_modal", "templates/modals/luis.hbs")?;
    content.register_template("cf_ea_modal", "templates/modals/cf-ea-revamp.hbs")?;

    info!("Removing old generated templates...");
    match std::fs::remove_dir_all("templates/generated") {
        Ok(_) => info!("Removed old generaetd templates."),
        Err(_) => info!("Old generated templates do not exist.")
    };
    
    info!("Creating new generated templates directory...");
    std::fs::create_dir("templates/generated")?;
    
    info!("Generating content...");
    content.generate("index", "templates/generated/index.html")?;
    content.generate("404", "templates/generated/404.html")?;
    
    Ok(())
}