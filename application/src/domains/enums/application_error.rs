use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    // #[error("Failed to deserialize JSON")]
    // JsonDeserializeError,
    #[error("Request failed")]
    RequestError
}
