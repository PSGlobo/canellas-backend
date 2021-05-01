use std::{net::TcpListener, str::FromStr};

use cotid_server::{config::Settings, run_app};
use sqlx::{postgres::PgConnectOptions, Connection, Executor, PgConnection, PgPool};

pub async fn spawn_test_app() -> String {
    dotenv::dotenv().ok();

    let mut settings = Settings::load().unwrap();
    settings.database.name = uuid::Uuid::new_v4().to_string();

    let listener = TcpListener::bind("127.0.0.1:0").expect("couldn't bind to random port");
    let port = listener.local_addr().unwrap().port();
    dbg!(&settings);
    let pool = config_db(&settings.database).await;
    dbg!(&pool);
    let app = run_app(listener, pool).unwrap();
    tokio::spawn(app);

    format!("http://127.0.0.1:{}", port)
}

async fn config_db(config: &cotid_server::config::Database) -> PgPool {
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
