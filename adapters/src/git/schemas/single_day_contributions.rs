#[cynic::schema_for_derives(file = r#"../graphql/github.graphql"#, module = "schemas")]
pub mod queries {
    use schemas;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct SingleDayContributionsVariables {
        pub date: DateTime,
        pub login: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SingleDayContributionsVariables")]
    pub struct SingleDayContributions {
        #[arguments(login: $login)]
        pub user: Option<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "SingleDayContributionsVariables")]
    pub struct User {
        #[arguments(from: $date, to: $date)]
        pub contributions_collection: ContributionsCollection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ContributionsCollection {
        pub contribution_calendar: ContributionCalendar,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ContributionCalendar {
        pub weeks: Vec<ContributionCalendarWeek>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ContributionCalendarWeek {
        pub contribution_days: Vec<ContributionCalendarDay>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ContributionCalendarDay {
        pub contribution_count: i32,
        pub date: Date,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Date(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTime(pub String);
}
