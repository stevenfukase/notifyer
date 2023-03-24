#[cynic::schema_for_derives(file = r#"../graphql/github.graphql"#, module = "schema")]
mod queries {
    use schemas as schema;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SingleDayComittedReposVariables {
        pub from: Option<DateTime>,
        pub login: Option<String>,
        pub to: Option<DateTime>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SingleDayComittedReposVariables")]
    pub struct SingleDayComittedRepos {
        #[arguments(login: $login)]
        pub user: Option<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "SingleDayComittedReposVariables")]
    pub struct User {
        #[arguments(from: $from, to: $to)]
        pub contributions_collection: ContributionsCollection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ContributionsCollection {
        pub commit_contributions_by_repository: Vec<CommitContributionsByRepository>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct CommitContributionsByRepository {
        #[arguments(last: 100, orderBy: { direction: "ASC", field: "COMMIT_COUNT" })]
        pub contributions: CreatedCommitContributionConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct CreatedCommitContributionConnection {
        pub nodes: Option<Vec<Option<CreatedCommitContribution>>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct CreatedCommitContribution {
        pub commit_count: i32,
        pub repository: Repository,
        pub occurred_at: DateTime,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Repository {
        pub name_with_owner: String,
        pub url: Uri,
        pub open_graph_image_url: Uri,
        pub primary_language: Option<Language>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Language {
        pub name: String,
        pub color: Option<String>,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTime(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    #[cynic(graphql_type = "URI")]
    pub struct Uri(pub String);
}
