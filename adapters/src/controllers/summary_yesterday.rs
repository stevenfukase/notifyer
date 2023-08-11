use crate::{presenters, shared::app_state::AppState};
use application::{
    domains::enums::application_error::ApplicationError,
    usecases::{
        abstract_usecase::AbstractUsecase,
        notify_yesterday_summary_usecase::NotifyYesterdaySummaryUsecase,
    },
};

pub async fn summary_yesterday(app_state: &AppState) -> Result<(), ApplicationError> {
    let notify_yesterday_summary_usecase =
        NotifyYesterdaySummaryUsecase::new(&app_state.git_repository);
    let summary = notify_yesterday_summary_usecase.execute().await?;
    let summary_presenter = presenters::summary::summary(&summary);

    app_state.messaging_service.send(summary_presenter).await
}
