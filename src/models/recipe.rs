//! Recipe model

use crate::errors::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

/// The Recipe object
#[derive(Queryable)]
pub struct Recipe {
    id: Uuid,
    title: String,
    source_url: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[juniper::object]
impl Recipe {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn source_url(&self) -> Option<&str> {
        self.source_url.as_ref().map(|s| s.as_str())
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn opengraph(&self) -> Option<OpenGraphInfo> {
        self.source_url.as_ref().and_then(|url| {
            webpage::Webpage::from_url(&url, webpage::WebpageOptions::default())
                .ok()
                .map(|page| OpenGraphInfo {
                    og: page.html.opengraph,
                })
        })
    }
}

/// OpenGraph info extracted from recipe's source URL
pub struct OpenGraphInfo {
    og: webpage::Opengraph,
}

#[juniper::object]
impl OpenGraphInfo {
    /// The website's title
    pub fn title(&self) -> Option<&String> {
        self.og.properties.get("title")
    }

    /// The website's site name
    pub fn site_name(&self) -> Option<&String> {
        self.og.properties.get("site_name")
    }

    /// The website's image
    pub fn image_url(&self) -> Option<&String> {
        self.og.images.get(0).map(|img| &img.url)
    }
}

/// Repository object for Recipes
pub struct Repo;

impl Repo {
    /// Query for Recipes with limit
    pub fn with_limit(connection: &diesel::PgConnection, limit: i64) -> Result<Vec<Recipe>> {
        use crate::schema::recipes::dsl::*;

        let results = recipes.limit(limit).load(connection)?;
        Ok(results)
    }
}
