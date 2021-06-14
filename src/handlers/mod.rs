//! # handlers
//!
//! Code that receives the requests from the router.
//!

pub mod dummy;
pub mod health_check;
pub mod users;

type Pool = actix_web::web::Data<sqlx::PgPool>;
