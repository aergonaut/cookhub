use juniper::FieldResult;

#[derive(juniper::GraphQLObject)]
pub struct Recipe {
    title: String,
    source_url: String,
}

pub struct Query;

#[juniper::object]
impl Query {
    fn recipes() -> FieldResult<Vec<Recipe>> {
        Ok(vec![])
    }
}

pub type Schema = juniper::RootNode<'static, Query, juniper::EmptyMutation<()>>;
