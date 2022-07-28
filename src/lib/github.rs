use chrono::Utc;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use serde_json::json;

type DateTime = String;
type Date = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/lib/graphql/github_schema.graphql",
    query_path = "src/lib/graphql/user_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct SingleDayContribution;

const GITHUB_ENDPOINT: &str = "https://api.github.com/graphql";

pub async fn todays_activity_count() -> Result<i64, reqwest::Error> {
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

    let variables = single_day_contribution::Variables {
        login: "stevenfukase".to_owned(),
        date: "2022-07-27T00:00:00Z".to_owned(),
    };
    // let variables = single_day_contribution::Variables {
    //     login: github_username.to_string(),
    //     date: Utc::now().to_rfc3339(),
    // };

    // graphql_client::reqwest::post_graphql will cause error
    // when compiling for armv7-unknown-linux-gnueabihf
    let response = client
        .post(GITHUB_ENDPOINT)
        .json(&SingleDayContribution::build_query(variables))
        .send()
        .await?
        .json::<single_day_contribution::ResponseData>()
        .await;

    println!("{:?}", response);

    Ok(0)

    // Ok(response
    //     .user
    //     .unwrap()
    //     .contributions_collection
    //     .contribution_calendar
    //     .weeks[0]
    //     .contribution_days[0]
    //     .contribution_count)
}
