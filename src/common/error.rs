use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /* Wrapper Errors */
    #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>),
    #[error("IO Error: {0}")] IOError(#[from] std::io::Error),
    #[error("TOML Error: {0}")] TOMLError(#[from] toml::de::Error),
    #[error("Lua Error: {0}")] LuaError(#[from] mlua::Error),
    #[error("WalkDir Error: {0}")] WDirError(#[from] walkdir::Error),
    #[error("StripPrefix: {0}")] StrPrefixError(#[from] std::path::StripPrefixError),
    #[error("Tera Error: {0}")] TeraError(#[from] tera::Error),

    /* Custom Errors */
    #[error("Invalid Template Structure: {0}")]
    TemplateStructureError(String),

    #[error("Type Missmatch: {0}")]
    TypeMissmatchError(String)
}

pub type Result<T> = std::result::Result<T, Error>;