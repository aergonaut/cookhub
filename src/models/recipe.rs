//! Recipe model

use crate::errors::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// The Recipe object
#[derive(Queryable, juniper::GraphQLObject)]
pub struct Recipe {
    id: Uuid,
    title: String,
    source_url: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Recipe {
    /// Query for Recipes with limit
    pub fn with_limit(connection: &diesel::PgConnection, limit: i64) -> Result<Vec<Recipe>> {
        use crate::schema::recipes::dsl::*;

        let results = recipes.limit(limit).load(connection)?;
        Ok(results)
    }
}
