use actix_web::{
    web::{self, post},
    ResponseError,
};
use anyhow::Context;
use sqlx::PgPool;

use crate::repos::{self, users::NewUser};

#[derive(Debug, thiserror::Error)]
enum UserError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ResponseError for UserError {}

#[tracing::instrument]
async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> Result<web::Json<serde_json::Value>, UserError> {
    let repo = repos::users::Repo::new(pool.as_ref());

    let id = repo
        .insert(&user.0)
        .await
        .context("Failed to insert user.")?;

    Ok(web::Json(serde_json::json!({
        "id": id,
        "email": user.email,
    })))
}

pub fn handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(post().to(create_user)));
}
