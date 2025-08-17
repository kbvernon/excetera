use extendr_api::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::BufWriter;
use tera::{Context, Tera};

#[derive(Debug)]
#[extendr]
struct ExTera(Tera);

#[extendr]
impl ExTera {
    fn new(template_glob: &str) -> ExTera {
        match Tera::new(template_glob) {
            Ok(tera) => ExTera(tera),
            Err(e) => throw_r_error(&format!(
                "Could not load templates from '{}': {}",
                template_glob, e
            )),
        }
    }

    fn add_str_templates(&mut self, templates: List) -> Rbool {
        let mut template_tuples: Vec<(&str, &str)> = vec![];

        for (name, content) in templates.iter() {
            template_tuples.push((name, &content.as_str().unwrap()))
        }

        match self.0.add_raw_templates(template_tuples) {
            Ok(_) => Rbool::true_value(),
            Err(e) => throw_r_error(&format!(
                "Could not add templates '{:?}': {}",
                templates.names().unwrap().collect::<Vec<_>>(),
                e
            )),
        }
    }

    fn add_file_templates(&mut self, templates: List) -> Rbool {
        let mut template_tuples: Vec<(&str, Option<&str>)> = vec![];

        for (name, content) in templates.iter() {
            let template_name = if name.is_empty() { None } else { Some(name) };
            template_tuples.push((&content.as_str().unwrap(), template_name))
        }

        match self.0.add_template_files(template_tuples) {
            Ok(_) => Rbool::true_value(),
            Err(e) => throw_r_error(&format!(
                "Could not add templates '{:?}': {}",
                templates.names().unwrap().collect::<Vec<_>>(),
                e
            )),
        }
    }

    fn list_templates(&self) -> Strings {
        self.0.get_template_names().collect()
    }

    fn render_to_file(
        &self,
        template_name: &str,
        parameters: HashMap<String, String>,
        outfile: &str,
    ) -> Rbool {
        let context = match Context::from_serialize(parameters) {
            Ok(ctx) => ctx,
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

        match self.0.render_to(template_name, &context, &mut writer) {
            Ok(_) => Rbool::true_value(),
            Err(e) => throw_r_error(&format!(
                "Unable to render template '{}' to file: {}",
                template_name, e
            )),
        }
    }

    /// Render a single template and return as string
    fn render_to_string(&self, template_name: &str, parameters: HashMap<String, String>) -> String {
        let context = match Context::from_serialize(parameters) {
            Ok(ctx) => ctx,
            Err(e) => throw_r_error(&format!("Failed to serialize context: {}", e)),
        };

        match self.0.render(template_name, &context) {
            Ok(result) => result,
            Err(e) => throw_r_error(&format!(
                "Unable to render template '{}': {}",
                template_name, e
            )),
        }
    }

    fn reload(&mut self) -> Rbool {
        match self.0.full_reload() {
            Ok(_) => Rbool::true_value(),
            Err(e) => throw_r_error(&format!("Could not reload templates: {}", e)),
        }
    }

    fn autoescape_on(&mut self) -> Rbool {
        self.0.reset_escape_fn();
        Rbool::true_value()
    }

    fn autoescape_off(&mut self) -> Rbool {
        self.0.autoescape_on(vec![]);
        Rbool::true_value()
    }
}

#[extendr]
fn render_template(template: &str, outfile: &str, parameters: HashMap<String, String>) -> Rbool {
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
        Ok(_) => Rbool::true_value(),
        Err(e) => throw_r_error(&format!("Unable to render template to file: {}", e)),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod excetera;
    fn render_template;
    impl ExTera;
}
