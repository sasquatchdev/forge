use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /* Wrapper Errors */
    #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>),
    #[error("IO Error: {0}")] IOError(#[from] std::io::Error),
    #[error("TOML Error: {0}")] TOMLError(#[from] toml::de::Error),
    
    /* Custom Errors */
    #[error("Invalid Template Structure: {0}")]
    TemplateStructureError(String)
}

pub type Result<T> = std::result::Result<T, Error>;