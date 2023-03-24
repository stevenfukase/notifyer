use super::schemas::single_day_contributions::queries::{
    DateTime, SingleDayContributions, SingleDayContributionsVariables,
};
use cynic::{Operation, QueryBuilder};

pub fn build_query(
    login: &str,
    date: &str,
) -> Operation<SingleDayContributions, SingleDayContributionsVariables> {
    SingleDayContributions::build(SingleDayContributionsVariables {
        date: DateTime(date.to_string()),
        login: login.to_string(),
    })
}
