use extendr_api::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::BufWriter;
use tera::{Context, Tera};

#[extendr]
fn render_template(template: &str, outfile: &str, parameters: HashMap<String, String>) -> Robj {
    let template_content = match fs::read_to_string(template) {
        Ok(content) => content,
        Err(e) => throw_r_error(&format!(
            "Could not read template file '{}': {}",
            template, e
        )),
    };

    let mut tera = Tera::default();

    if let Err(e) = tera.add_raw_template("template", &template_content) {
        throw_r_error(&format!("Could not add template: {}", e));
    }

    let context = match Context::from_serialize(parameters) {
        Ok(t) => t,
        Err(e) => throw_r_error(&format!("Failed to serialize context: {}", e)),
    };

    let file = match fs::File::create(outfile) {
        Ok(f) => f,
        Err(e) => throw_r_error(&format!(
            "Could not create output file '{}': {}",
            outfile, e
        )),
    };

    let mut writer = BufWriter::new(file);

    match tera.render_to("template", &context, &mut writer) {
        Ok(_) => Rbool::true_value().into(),
        Err(e) => throw_r_error(&format!("Unable to render template to file: {}", e)),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod excetera;
    fn render_template;
}
