use std::path::PathBuf;

use crate::{files::read_dirs, template::{Template, VariantConfiguration}, Result};

pub fn select_from<T>(message: impl Into<String>, choices: Vec<T>) -> T
    where T: ToString
{
    let strings = choices.iter().map(|c| c.to_string()).collect();
    let selected = inquire::Select::new(&message.into(), strings)
        .prompt()
        .unwrap();
    choices.into_iter().find(|c| c.to_string() == selected).unwrap()
}

pub fn select_template(message: impl Into<String>, path: impl Into<PathBuf>) -> Result<Template> {
    let templates: Vec<Template> = read_dirs(path.into())?
        .into_iter()
        .map(|path| Template::load(path))
        .filter_map(Result::ok)
        .collect();

    Ok(select_from(message.into(), templates))
}

pub fn select_variant(message: impl Into<String>, template: &Template) -> Result<VariantConfiguration> {
    let variants = &template.config.variant;
    if variants.len() == 1 {
        return Ok(variants[0].clone());
    }
    Ok(select_from(message.into(), variants.to_vec()))
}