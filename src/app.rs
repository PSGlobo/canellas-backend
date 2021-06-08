//! App Startup

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, get, Data},
    HttpServer,
};
use sqlx::PgPool;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;

use crate::{
    config::Settings,
    db::create_pool,
    handlers::{
        dummy::{db_test, hello},
        health_check::health_check,
    },
};

/// Holds information related to the server
pub struct App {
    port: u16,
    server: Server,
}

impl App {
    /// Creates an [App] based on the settings, initializing and configuring the its dependencies.
    pub async fn build(settings: &Settings) -> Result<Self, std::io::Error> {
        let pool = create_pool(&settings.database).await;
        info!("Database connected");
        debug!("{:?}", &pool);

        let listener = TcpListener::bind((settings.host.clone(), settings.port))?;

        let addr = listener.local_addr().unwrap();
        info!(
            "Server's connection listener set up on {}:{}",
            addr.ip(),
            addr.port()
        );

        Ok(Self {
            port: addr.port(),
            server: build_server(listener, pool)?,
        })
    }

    /// Starts the [App]. Nothing happens until you await for it.
    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    /// Get a reference to the app's port.
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Creates, configures and starts an instance of the server.
fn build_server(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(pool);
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(TracingLogger)
            .app_data(pool.clone())
            .route("/", get().to(hello))
            .route("/health_check", get().to(health_check))
            .service(
                web::scope("/test/")
                    .route("/db", get().to(db_test))
                    .route("/{name}", get().to(hello)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
