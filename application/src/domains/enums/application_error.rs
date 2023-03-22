use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
}
