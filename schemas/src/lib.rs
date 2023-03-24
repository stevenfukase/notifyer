cynic::use_schema!("../graphql/github.graphql");

cynic::impl_scalar!(chrono::DateTime<chrono::Utc>, DateTime);
