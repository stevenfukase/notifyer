use graphql_client::GraphQLQuery;
use reqwest::Client;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/graphql/github_schema.graphql",
    query_path = "src/lib/graphql/user_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct UserContributions;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn get_activity() -> Result<user_contributions::ResponseData, reqwest::Error> {
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

    let variables = user_contributions::Variables {
        login: github_username.to_string(),
    };

    let request_body = UserContributions::build_query(variables);

    // graphql_client::reqwest::post_graphql will cause error
    // when compiling for armv7-unknown-linux-gnueabihf

    let response = client
        .post(GITHUB_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;
    println!("{:?}", response);
    
    let parsed_response = response.json().await?;
    Ok(parsed_response)
}
