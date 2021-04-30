use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

// define a custom helper
pub fn to_lower_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{}", param.value().render()).to_lowercase();
    out.write(rendered.as_ref())?;
    Ok(())
}