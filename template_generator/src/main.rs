mod content;

use std::error::Error;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    info!("Loading data...");
    let mut content = content::Content::init("content.json");

    info!("Loading templates...");
    content.register_template("base", "templates/base.hbs")?;
    content.register_template("text-modal-base", "templates/modals/base/text-modal-base.hbs")?;
    content.register_template("image-modal-base", "templates/modals/base/image-modal-base.hbs")?;

    content.register_template("index", "templates/index.hbs")?;
    content.register_template("404", "templates/404.hbs")?;

    content.register_template("al22_modal", "templates/modals/content/al22.hbs")?;
    content.register_template("cs128h_modal", "templates/modals/content/cs128h.hbs")?;
    content.register_template("twilio_modal", "templates/modals/content/twilio.hbs")?;
    content.register_template("kqueue_modal", "templates/modals/content/kqueue.hbs")?;
    content.register_template("burd_modal", "templates/modals/content/burd.hbs")?;
    content.register_template("luis_modal", "templates/modals/content/luis.hbs")?;
    content.register_template("cf_ea_modal", "templates/modals/content/cf-ea-revamp.hbs")?;

    info!("Removing old generated templates...");
    match std::fs::remove_dir_all("templates/generated") {
        Ok(_) => info!("Removed old generated templates."),
        Err(_) => info!("Old generated templates do not exist.")
    };
    
    info!("Creating new generated templates directory...");
    std::fs::create_dir("templates/generated")?;
    
    info!("Generating content...");
    content.generate("index", "templates/generated/index.html")?;
    content.generate("404", "templates/generated/404.html")?;
    
    Ok(())
}