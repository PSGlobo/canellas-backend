//! DB communication bootstrap

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::Database;

/// Returns a configured [PgPool] ready to be used to interact with the DBMS.
pub async fn create_pool(setting: &Database) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect(&setting.uri())
        .await
        .expect("Couldn't connect with DB")
}
