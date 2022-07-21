use graphql_client::GraphQLQuery;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/github/github_schema.graphql",
    query_path = "src/lib/github/user_contributions.graphql",
    response_derives = "Serialize,PartialEq"
)]
pub struct UserContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn get_activity() {
    let github_username = env!("GITHUB_USERNAME").to_owned();
    let variables = user_contributions::Variables { login: github_username };
    let request_body = UserContributions::build_query(variables);
}
