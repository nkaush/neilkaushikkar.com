extern crate handlebars;

use handlebars::{
    Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

// define a custom helper
pub fn format_skill(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let percent = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;

    let skill = h
        .param(1)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    
    let rendered: String = format!(
        "<div class=\"skill\">
            <span class=\"chart\" data-percent=\"91\">
                <span class=\"percent\">{}</span>
                <canvas height=\"152\" width=\"152\"></canvas>
            </span>
            <h4>{}</h4>
        </div>", percent.value().render(), skill.value().render()
    );
    
    out.write(rendered.as_ref())?;
    Ok(())
}