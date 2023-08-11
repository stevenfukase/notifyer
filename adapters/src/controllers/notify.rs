use crate::{presenters, shared::app_state::AppState};
use application::{
    domains::enums::application_error::ApplicationError,
    usecases::{
        abstract_usecase::AbstractUsecase, notify_no_commit_usecase::NotifyNoCommitUsecase,
    },
};

pub async fn notify(app_state: &AppState) -> Result<(), ApplicationError> {
    let notify_no_commit_usecase = NotifyNoCommitUsecase::new(&app_state.git_repository);
    let need_to_notify = notify_no_commit_usecase.execute().await?;
    if need_to_notify {
        let notify_presenter = presenters::notify::notify();
        app_state.messaging_service.send(notify_presenter).await?
    }

    Ok(())
}
