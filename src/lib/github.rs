const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

use graphql_client::GraphQLQuery;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/github_schema.graphql",
    query_path = "src/lib/user_contributions.graphql",
    response_derives = "Serialize,PartialEq"
)]
pub struct UserContributions;

pub fn get_today_activity() {}
