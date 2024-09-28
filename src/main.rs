mod common;

pub use common::*;
pub use common::error::*;
use interact::{select_template, select_variant};

mod template;
mod interact;

const TEMPLATES: &str = "/home/sasquatchdev/forge/res";
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let template = select_template("Please select a template", TEMPLATES)?;
    let variant = select_variant("Please select a variant", &template)?;
    
    println!("{}", variant.files.join(", "));
    Ok(())
}
