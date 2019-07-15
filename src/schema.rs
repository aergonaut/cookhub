table! {
    recipes (id) {
        id -> Uuid,
        name -> Varchar,
        source_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
