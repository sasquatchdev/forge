mod common;

use std::path::PathBuf;

pub use common::*;
pub use common::error::*;
use interact::{select_template, select_variant};
use script::eval_from_all;

mod template;
mod interact;
mod script;

const TEMPLATES: &str = "/home/sasquatchdev/forge/res";
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 1. Select template & variant
    let template = select_template("Please select a template", TEMPLATES)?;
    let variant = select_variant("Please select a variant", &template)?;
    
    // 2. Gather user input
    let inputs = variant.input
        .iter()
        .map(|str| template.path.join(PathBuf::from(str)))
        .collect();

    let _values = eval_from_all(inputs);
    Ok(())
}
