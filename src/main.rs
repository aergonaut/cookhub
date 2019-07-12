#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::content;
use rocket::State;

mod graphql;

#[rocket::get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::post("/graphql", data = "<request>")]
fn graphql_query(
    request: juniper_rocket::GraphQLRequest,
    schema: State<graphql::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &())
}

fn main() {
    rocket::ignite()
        .manage(graphql::Schema::new(
            graphql::Query,
            juniper::EmptyMutation::new(),
        ))
        .mount("/", rocket::routes![graphiql, graphql_query])
        .launch();
}
