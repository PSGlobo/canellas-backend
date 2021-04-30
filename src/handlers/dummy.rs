//! # Dummy
//!
//! Endpoints to test the service execution.
//!

use actix_web::{web, HttpRequest, Responder};
use sqlx::{query_as, PgPool};

/// Hello, world!
#[tracing::instrument(name = "Hello World", skip(req))]
pub async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// TODO remove it when this test endpoint is no longer needed
#[tracing::instrument]
pub async fn db_test(pool: web::Data<PgPool>) -> impl Responder {
    let rows = query_as::<_, (String,)>("SELECT * FROM test")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    format!("{:?}", rows)
}
