//! Types for the GraphQL API

use juniper::FieldResult;

/// The Recipe object
#[derive(juniper::GraphQLObject)]
pub struct Recipe {
    title: String,
    source_url: String,
}

/// The root Query object
pub struct Query;

#[juniper::object]
impl Query {
    fn recipes() -> FieldResult<Vec<Recipe>> {
        Ok(vec![])
    }
}

/// The Cookhub GraphQL schema type
pub type Schema = juniper::RootNode<'static, Query, juniper::EmptyMutation<()>>;
