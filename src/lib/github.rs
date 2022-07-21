use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/graphql/github_schema.graphql",
    query_path = "src/lib/graphql/user_contributions.graphql",
    response_derives = "Serialize,PartialEq"
)]
pub struct UserContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn get_activity(
) -> Result<graphql_client::Response<user_contributions::ResponseData>, reqwest::Error> {
    let client = Client::builder().build()?;

    let github_username = env!("GITHUB_USERNAME").to_owned();
    let variables = user_contributions::Variables {
        login: github_username,
    };
    post_graphql::<UserContributions, _>(&client, GITHUB_ENDPOINT, variables).await
}
