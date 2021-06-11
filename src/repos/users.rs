#![allow(missing_docs)]

#[derive(Debug, serde::Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub struct Repo<'a> {
    pool: &'a sqlx::PgPool,
}

pub type Id = String;

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
        Ok(rec.id.to_string())
    }
}
