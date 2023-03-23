use crate::{infrastructure::app_state::AppState, presenters};
use application::{
    domains::enums::application_error::ApplicationError,
    usecases::{abstract_usecase::AbstractUsecase, notify_summary_usecase::NotifySummaryUsecase},
};

pub async fn summary(app_state: &AppState) -> Result<(), ApplicationError> {
    let notify_summary_usecase = NotifySummaryUsecase::new(&app_state.git_repository);
    let (date_time, contributed_repositories) = notify_summary_usecase.execute().await?;
    let summary_presenter = presenters::summary::summary(&contributed_repositories, &date_time);

    app_state.messaging_service.send(summary_presenter).await
}
