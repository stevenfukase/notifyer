use crate::git::schemas::single_day_comitted_repos::queries::{
    DateTime, SingleDayCommittedRepos, SingleDayCommittedReposVariables,
};
use cynic::{Operation, QueryBuilder};

pub fn build_query(
    login: &str,
    date: &str,
) -> Operation<SingleDayCommittedRepos, SingleDayCommittedReposVariables> {
    SingleDayCommittedRepos::build(SingleDayCommittedReposVariables {
        date: DateTime(date.to_string()),
        login: login.to_string(),
    })
}
