mod common;

use std::{fs, path::PathBuf};

pub use common::*;
pub use common::error::*;
use inquire::Text;
use interact::{select_template, select_variant};
use render::render_dir;
use script::eval_from_all;
use tera::Context;

mod template;
mod interact;
mod render;
mod script;

const TEMPLATES: &str = "/home/sasquatchdev/forge/res";
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 1. Select template & variant
    let template = select_template("Please select a template", TEMPLATES)?;
    let variant = select_variant("Please select a variant", &template)?;
    let title = Text::new("Please provide a title").prompt()?;
    let path = Text::new("Please provide a path").with_default(&format!("./{}", title)).prompt()?;
    let target = PathBuf::from(path);

    // 2. Gather user input
    let inputs = variant.input
        .iter()
        .map(|str| template.path.join(PathBuf::from(str)))
        .collect();

    let values: Vec<serde_json::Value> = eval_from_all(inputs).into_iter()
        .filter(|v| v.is_object())
        .collect();
    
    // 3. Prepare Context
    // 3.1. Insert Defaults
    let mut context = Context::new();
    context.insert("title", &title);

    // 3.2. Combine Tables
    let mut combined = values[0].clone();
    for value in values {
        script::json::merge_json(&mut combined, value);
    }

    // 3.3. Insert Dynamics
    if let serde_json::Value::Object(object) = combined {
        for (k, v) in object {
            context.insert(k, &v);
        }
    }

    // 4. Render files
    let files: Vec<PathBuf> = variant.files
        .iter()
        .map(|str| PathBuf::from(str))
        .collect();

    fs::create_dir_all(&target)?;
    for dir in files {
        render_dir(template.path.join(&dir), target.clone(), &context).unwrap();
    }

    Ok(())
}
