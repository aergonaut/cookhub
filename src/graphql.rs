//! Types for the GraphQL API

use crate::models::*;
use juniper::FieldResult;

/// The root Query object
pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn recipes(context: &Context) -> FieldResult<Vec<Recipe>> {
        let conn = &*context.db.get()?;
        match Recipe::with_limit(conn, 100) {
            Ok(results) => Ok(results),
            Err(e) => Err(e.to_string())?,
        }
    }
}

/// Context object for GraphQL queries. Provides access to the DB pool.
pub struct Context {
    /// R2D2 connection pool
    pub db: crate::db::PgConnectionPool,
}

impl juniper::Context for Context {}

/// The Cookhub GraphQL schema type
pub type Schema = juniper::RootNode<'static, Query, juniper::EmptyMutation<Context>>;
