//! Types and functions relating to the database connection

use crate::errors::Result;

type PgConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;

/// Convenience type for a pooled Postgres connection
pub type PgConnectionPool = diesel::r2d2::Pool<PgConnectionManager>;

/// Create the DB connection pool
pub fn establish_connection_pool(url: &str) -> Result<PgConnectionPool> {
    let manager = PgConnectionManager::new(url);
    let pool = diesel::r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
