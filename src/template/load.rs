use std::{fs, path::PathBuf};
use crate::{Error, Result};
use super::{Configuration, Template};

const CONFIG_NAME: &str = "Forge.toml";
impl Template {
    pub fn new(identifier: String, path: PathBuf, config: Configuration) -> Self {
        Self { identifier, path, config }
    }

    pub fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path: PathBuf = path.into();
        if path.is_dir() {
            /* If a directory is passed, assume it's the root */
            Self::load_from_root(&path)
        } else {
            /* If a file is passed, assume it's the config */
            Self::load_from_config(&path)
        }
    }

    pub fn load_config(path: &PathBuf) -> Result<Configuration> {
        let content = fs::read_to_string(path)?;
        let parsed = toml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn load_from_config(path: &PathBuf) -> Result<Self> {
        let config = Self::load_config(&path)?;

        let path = path.parent().expect("Cannot load templates from the root directory!").to_path_buf();
        let ident = path.file_stem().unwrap().to_str().unwrap().to_string();
        
        Ok(Self::new(ident, path, config))
    }

    pub fn load_from_root(path: &PathBuf) -> Result<Self> {
        let config = path.join(CONFIG_NAME);
        if !config.exists() {
            return Err(Error::TemplateStructureError(
                format!("Could not find {} in {:?}!", CONFIG_NAME, path)
            ));
        }

        Ok(Self::load_from_config(&config)?)
    }
}