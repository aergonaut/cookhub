//! Utility module for Tera template helper functions.

use rocket_contrib::templates::tera as t;
use std::collections::HashMap;

/// Factory function to create the `asset_path` helper function for Tera templates.
pub fn make_asset_path_function(manifest: HashMap<String, String>) -> t::GlobalFn {
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
