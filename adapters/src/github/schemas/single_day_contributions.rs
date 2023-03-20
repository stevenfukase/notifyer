use graphql_client::GraphQLQuery;

type DateTime = String;
type Date = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/services/github/schemas/graphql/github_schema.graphql",
    query_path = "src/services/github/schemas/graphql/single_day_contributions.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct SingleDayContributions;
