use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod load;
pub mod display;

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub identifier: String,
    pub path: PathBuf,
    pub config: Configuration,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub display: DisplayConfiguration,
    pub internal: Option<InternalConfiguration>,
    pub variant: Vec<VariantConfiguration>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DisplayConfiguration {
    pub name: String,
    pub version: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InternalConfiguration {
    /* Forge version requirement using `semver` */
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VariantConfiguration {
    pub name: String,
    pub files: Vec<String>,
}