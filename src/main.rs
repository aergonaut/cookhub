//! Cookhub application entrypoint

#![feature(proc_macro_hygiene, decl_macro)]
#![deny(missing_docs, unsafe_code, clippy::missing_docs_in_private_items)]

use rocket::response::content;
use rocket::State;
use rocket_contrib::templates::tera as t;
use rocket_contrib::templates::{Engines, Template};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

mod graphql;

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
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &())
}

/// GET handler for root path
#[rocket::get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Hello world!");
    Template::render("index", context)
}

/// Factory function to create the `asset_path` helper function for Tera templates.
fn make_asset_path_function(manifest: HashMap<String, String>) -> t::GlobalFn {
    Box::new(move |args| -> t::Result<t::Value> {
        match args.get("name") {
            Some(val) => match t::from_value::<String>(val.clone()) {
                Ok(name) => Ok(t::to_value(manifest.get(&name).unwrap()).unwrap()),
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

/// The main function's responsibility is to instantiate the Rocket instance and attach all of the
/// route handlers, state, middleware, etc.
fn main() {
    rocket::ignite()
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
                .register_function("asset_path", make_asset_path_function(manifest))
        }))
        .mount("/", rocket_contrib::serve::StaticFiles::from("public"))
        .mount("/", rocket::routes![index, graphiql, graphql_query])
        .launch();
}
