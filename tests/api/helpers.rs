use std::str::FromStr;

use ps_globo::{
    app::App,
    config::{self, Settings},
};
use sqlx::{postgres::PgConnectOptions, Connection, Executor, PgConnection, PgPool};

pub async fn test_client() -> TestClient {
    let host = spawn_test_app().await;

    TestClient {
        inner: reqwest::Client::new(),
        host,
    }
}

pub struct TestClient {
    inner: reqwest::Client,
    host: String,
}

impl TestClient {
    pub async fn new() -> TestClient {
        test_client().await
    }

    pub fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: &impl serde::Serialize,
    ) -> reqwest::RequestBuilder {
        self.inner
            .request(method, &format!("{}/{}", self.host, path))
            .json(body)
    }
}

/// Returns a `String` representing the host and the port used by the service.
pub async fn spawn_test_app() -> String {
    dotenv::dotenv().ok();

    let settings = {
        let mut s = Settings::load().unwrap();
        // Use different DB for each test
        s.database.name = uuid::Uuid::new_v4().to_string();
        // Use random port
        s.port = 0;
        s
    };

    config_db(&settings.database).await;

    let app = App::build(&settings).await.unwrap();
    let address = format!("http://127.0.0.1:{}", app.port());
    tokio::spawn(app.run());

    address
}

async fn config_db(config: &config::Database) -> PgPool {
    // Create DB
    dbg!(config.uri());
    let options = PgConnectOptions::from_str(&config.uri())
        .expect("Connection URI is not correct")
        .database("postgres"); // This is a maintainance database from postgres.

    let mut connection = PgConnection::connect_with(&options)
        .await
        .expect("Is the database running? Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.name))
        .await
        .expect("Failed to create database");

    // Migrate DB
    let connection_pool = PgPool::connect(&config.uri())
        .await
        .expect("Is the database running? Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
