mod common;
pub use common::*;
pub use common::error::*;
use template::Template;

mod template;

fn main() {
    let template = Template::load("./res/Forge.toml").unwrap();
    println!("Successfully loaded '{}' with {} variants!", template.config.display.name, template.config.variant.len());
}
