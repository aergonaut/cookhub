//! Utilty module for generating Rocket configuration

use crate::errors::Result;
use rocket::config::{Config, Environment};

/// Generate Rocket configuration object by reading canonical environment variables
pub fn make_config() -> Result<Config> {
    let address = "0.0.0.0";

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let workers = std::env::var("WEB_CONCURRENCY")
        .ok()
        .and_then(|w| w.parse().ok())
        .unwrap_or(4);

    let config = Config::build(Environment::active()?)
        .address(address.to_string())
        .port(port)
        .workers(workers)
        .finalize()?;

    Ok(config)
}

/// Derive the env filename from the current environment
pub fn env_file() -> Result<String> {
    Ok(format!(".env.{}", Environment::active()?.to_string()))
}
