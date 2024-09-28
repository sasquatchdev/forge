use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub identifier: String,
    pub version: String,
    pub config: Configuration,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub display: DisplayConfiguration,
    pub internal: Option<InternalConfiguration>,
    pub variants: HashMap<String, VariantConfiguration>,
}

#[derive(Serialize, Deserialize)]
pub struct DisplayConfiguration {
    pub name: String,
    pub version: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct InternalConfiguration {
    /* Forge version requirement using `semver` */
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct VariantConfiguration {
    pub files: Vec<String>,
}