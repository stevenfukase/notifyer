use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait GitRepositoryAbstract {
    async fn todo(&self) -> Result<(), Box<dyn std::error::Error>>;
}
