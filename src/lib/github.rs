use graphql_client::GraphQLQuery;
use reqwest::Client;

type DateTime = String;
type Date = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/graphql/github_schema.graphql",
    query_path = "src/lib/graphql/user_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct UserContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn todays_activity_count() -> Result<reqwest::Response, reqwest::Error> {
    let github_username = env!("GITHUB_USERNAME");
    let github_token = env!("GITHUB_ACCESS_TOKEN");

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let login = github_username.to_string();
    let variables = user_contributions::Variables { login };

    // graphql_client::reqwest::post_graphql will cause error
    // when compiling for armv7-unknown-linux-gnueabihf
    client
        .post(GITHUB_ENDPOINT)
        .json(&UserContributions::build_query(variables))
        .send()
        .await
}
