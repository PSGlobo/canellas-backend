use actix_web::{get, post, web, HttpResponse, ResponseError};
use anyhow::Context;
use repos::users::Repo;

use crate::repos::{
    self,
    users::{NewUser, User},
};

use super::Pool;

#[derive(Debug, thiserror::Error)]
enum UserError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ResponseError for UserError {}

#[tracing::instrument]
#[post("")]
async fn create_user(
    pool: Pool,
    user: web::Json<NewUser>,
) -> Result<web::Json<serde_json::Value>, UserError> {
    let repo = Repo::new(pool.as_ref());

    let id = repo
        .insert(&user.0)
        .await
        .context("Failed to insert user.")?;

    Ok(web::Json(serde_json::json!({
        "id": id,
        "email": user.email,
    })))
}

#[tracing::instrument]
#[get("/{id}")]
async fn get_user(pool: Pool, path: web::Path<i32>) -> Result<web::Json<User>, actix_web::Error> {
    let id = path.0;

    let user = Repo::new(&pool)
        .get_by_id(id)
        .await
        .map_err(|_| HttpResponse::NotFound())?;

    Ok(web::Json(User {
        id,
        email: user.email,
    }))
}

pub fn handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(create_user).service(get_user));
}
