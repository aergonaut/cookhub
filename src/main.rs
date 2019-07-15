//! Cookhub application entrypoint

#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_docs, unsafe_code, clippy::missing_docs_in_private_items)]

#[macro_use]
extern crate diesel;

use crate::errors::Result;
use rocket::response::content;
use rocket::State;
use rocket_contrib::templates::{Engines, Template};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

mod config;
mod db;
mod errors;
mod graphql;
mod helpers;
mod models;
#[allow(missing_docs)]
mod schema;

/// GET handler to serve GraphiQL
#[rocket::get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

/// POST handler for GraphQL requests
#[rocket::post("/graphql", data = "<request>")]
fn graphql_query(
    request: juniper_rocket::GraphQLRequest,
    schema: State<graphql::Schema>,
    db_pool: State<db::PgConnectionPool>,
) -> juniper_rocket::GraphQLResponse {
    let context = graphql::Context {
        db: db_pool.clone(),
    };
    request.execute(&schema, &context)
}

/// GET handler for root path
#[rocket::get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Hello world!");
    Template::render("index", context)
}

/// The main function's responsibility is to instantiate the Rocket instance and attach all of the
/// route handlers, state, middleware, etc.
fn main() -> Result<()> {
    dotenv::from_filename(&config::env_file()?)?;

    let config = config::make_config()?;

    rocket::custom(config)
        .manage(db::establish_connection_pool(&std::env::var(
            "DATABASE_URL",
        )?)?)
        .manage(graphql::Schema::new(
            graphql::Query,
            juniper::EmptyMutation::new(),
        ))
        .attach(Template::custom(|engines: &mut Engines| {
            let file = File::open("public/mix-manifest.json").unwrap();
            let buffer = BufReader::new(file);
            let manifest = serde_json::from_reader(buffer).unwrap();

            engines
                .tera
                .register_function("asset_path", helpers::make_asset_path_function(manifest))
        }))
        .mount("/", rocket_contrib::serve::StaticFiles::from("public"))
        .mount("/", rocket::routes![index, graphiql, graphql_query])
        .launch();

    Ok(())
}
