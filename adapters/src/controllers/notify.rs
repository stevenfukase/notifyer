use crate::infrastructure::app_state::AppState;
use application::domains::enums::application_error::ApplicationError;

pub async fn notify(app_state: &AppState) -> Result<(), ApplicationError> {
    todo!()
}
