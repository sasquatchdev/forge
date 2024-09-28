use std::{fs, path::PathBuf};

use crate::Result;

pub fn read_dirs(root: impl Into<PathBuf>) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(root.into())?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();

    Ok(entries)
}