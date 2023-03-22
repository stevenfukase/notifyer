use crate::domains::value_objects::message::Message;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MessagingRepositoryAbstract {
    async fn send(&self, message_body: &Message) -> Result<(), Box<dyn std::error::Error>>;
}
