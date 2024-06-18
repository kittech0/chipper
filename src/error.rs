use std::env::VarError;
use std::error::Error;

use log::error;
use thiserror::Error;

pub type BoxResult<T = ()> = Result<T, BoxError>;

#[derive(Error, Debug)]
pub enum BoxError {
    #[error("Expected token")]
    ExpectedToken(#[from] VarError),
    #[error("Client error")]
    ClientError(#[from] serenity::Error),
    #[error("Database error")]
    DBError(#[from] sled::Error),
}

pub fn error_print<E: Error>(err: E) {
    error!("{err}: {err:?}")
}
