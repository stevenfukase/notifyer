#[allow(non_snake_case, non_camel_case_types)]
cynic::use_schema!("../graphql/github.graphql");

cynic::impl_scalar!(chrono::DateTime<chrono::Utc>, DateTime);
