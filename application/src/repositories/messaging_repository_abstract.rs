use crate::domains::{enums::application_error::ApplicationError, value_objects::message::Message};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MessagingRepositoryAbstract {
    async fn send(&self, message_body: &Message) -> Result<(), ApplicationError>;
}
