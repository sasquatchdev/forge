use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>)
}

pub type Result<T> = std::result::Result<T, Error>;