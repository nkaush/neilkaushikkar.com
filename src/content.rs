use std::error::Error;
use serde_json::{Value, Map};
use handlebars::Handlebars;

pub struct Content<'a> {
    data: Map<String, Value>,
    handlebars: Handlebars<'a>
}

impl Content<'_> {
    pub fn init(data_path: &str) -> Self {
        let json: String = std::fs::read_to_string(data_path).unwrap();
        let data: Value = serde_json::from_str(&json).unwrap();
        let data: Map<String, Value> = data.as_object().unwrap().clone();
        
        Content { 
            data,
            handlebars: Handlebars::new()
        }
    }

    pub fn register_template(&mut self, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
        self.handlebars.register_template_file(name, path)?;
        Ok(())
    }

    pub fn generate(&self, name: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        let file_content = self.handlebars.render(name, &self.data)?;
        std::fs::write(output_path, file_content)?;
        Ok(())
    }
}