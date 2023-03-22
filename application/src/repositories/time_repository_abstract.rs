use crate::domains::value_objects::date_time::DateTime;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait TimeRepositoryAbstract {
    fn get_date<T>(&self, is_yesterday: bool) -> DateTime<T>;
}
