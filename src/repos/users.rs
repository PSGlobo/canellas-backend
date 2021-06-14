#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Id,
    pub email: String,
}

pub struct Repo<'a> {
    pool: &'a sqlx::PgPool,
}

pub type Id = i32;

impl<'a> Repo<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, user: &NewUser) -> Result<Id, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id"#,
            user.email,
            user.password
        )
        .fetch_one(self.pool)
        .await?;
        Ok(rec.id)
    }

    pub async fn get_by_id(&self, id: Id) -> Result<User, sqlx::Error> {
        let rec = sqlx::query_as!(User, "SELECT id, email FROM users WHERE id = $1", id)
            .fetch_one(self.pool)
            .await?;
        Ok(rec)
    }
}
