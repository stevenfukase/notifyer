use crate::{infrastructure::app_state::AppState, presenters};
use application::{
    domains::enums::application_error::ApplicationError,
    usecases::notify_yesterday_summary_usecase::NotifyYesterdaySummaryUsecase,
};

pub async fn summary_yesterday(app_state: &AppState) -> Result<(), ApplicationError> {
    let notify_yesterday_summary_usecase =
        NotifyYesterdaySummaryUsecase::new(&app_state.git_repository);
    let (date_time, contributed_repositories) = notify_yesterday_summary_usecase.execute().await?;
    let summary_presenter = presenters::summary::summary(&contributed_repositories, &date_time);

    app_state.messaging_service.send(summary_presenter).await
}
