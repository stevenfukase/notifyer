use graphql_client::GraphQLQuery;

type DateTime = String;
#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/services/github/schemas/graphql/github_schema.graphql",
    query_path = "src/services/github/schemas/graphql/single_day_committed_repo.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
pub struct SingleDayCommittedRepo;
