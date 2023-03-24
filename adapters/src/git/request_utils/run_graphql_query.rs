use cynic::{
    http::{CynicReqwestError, ReqwestExt},
    GraphQlResponse, Operation,
};

pub async fn run_graphql_query<ResponseData, Vars>(
    bearer_token: &str,
    endpoint: &str,
    operation: Operation<ResponseData, Vars>,
) -> Result<GraphQlResponse<ResponseData>, CynicReqwestError>
where
    Vars: serde::Serialize,
    ResponseData: serde::de::DeserializeOwned + 'static,
{
    let client = reqwest::Client::new();

    client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", bearer_token))
        .header("User-Agent", "notifyer")
        .run_graphql(operation)
        .await
}
