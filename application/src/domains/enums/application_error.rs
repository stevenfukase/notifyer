use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to deserialize")]
    DeserializeError,
    #[error("Request failed")]
    RequestError,
}
