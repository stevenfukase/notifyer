use cynic::{Operation, QueryFragment};

#[non_exhaustive]
pub struct GraphqlClient<QueryFragment, Vars = ()> {
    pub operation: Operation<QueryFragment, Vars>,
}

impl<QueryFragment, Vars> GraphqlClient<QueryFragment, Vars> {
    fn build() -> Self {}
}
