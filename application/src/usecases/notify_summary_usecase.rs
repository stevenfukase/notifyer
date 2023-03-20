use crate::domains::enums::application_error::ApplicationError;

use super::abstract_usecase::AbstractUsecase;
use async_trait::async_trait;

#[non_exhaustive]
pub struct NotifySummaryUsecase {
    is_yesterday: bool,
}

impl NotifySummaryUsecase {
    pub fn new(is_yesterday: bool) -> Self {
        Self { is_yesterday }
    }
}

#[async_trait(?Send)]
impl AbstractUsecase<()> for NotifySummaryUsecase {
    async fn execute() -> Result<(), ApplicationError> {
        Ok(())
    }
}
