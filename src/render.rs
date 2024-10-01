#![allow(unused)]

use std::{fs, path::PathBuf};

use tera::{Context, Tera};
use walkdir::WalkDir;

use crate::{Error, Result};

pub fn render_path(from: PathBuf, to: PathBuf, context: &Context) -> Result<()> {
    if from.is_file() && to.is_file() {
        render_file(from, to, context)
    } else if from.is_dir() && to.is_dir() {
        render_dir(from, to, context)
    } else {
        return Err(Error::TypeMissmatchError(if from.is_file() {
            format!(
                "Cannot render file ({:?}) into directory ({:?})",
                from, to
            )
        } else {
            format!(
                "Cannot render directory ({:?}) into file ({:?})",
                from, to
            )
        }))
    }
}

pub fn render_dir(from_root: PathBuf, to_root: PathBuf, context: &Context) -> Result<()> {
    for entry in WalkDir::new(&from_root)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_file())
    {
        let relative = entry.path().strip_prefix(&from_root)?.to_path_buf();
        render_file(from_root.join(&relative), to_root.join(relative), context)?;
    }
    Ok(())
}

pub fn render_file(from: PathBuf, mut to: PathBuf, context: &Context) -> Result<()> {
    if to.extension().map_or(false, |ext| ext == "to") {
        to = to.with_extension("");
    }
    
    let content = fs::read_to_string(from)?;
    let rendered = render(content, context)?;
    let _ = fs::create_dir_all(to.parent().unwrap());
    fs::write(to, rendered)?;
    Ok(())
}

pub fn render(content: String, context: &Context) -> Result<String> {
    let mut tera = Tera::default();
    let rendered = tera.render_str(&content, context)?;
    Ok(rendered)
}