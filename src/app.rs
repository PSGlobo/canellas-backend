//! App Startup

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, get, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::handlers::{
    dummy::{db_test, hello},
    health_check::health_check,
};

/// Creates, configures and starts an instance of the server.
pub fn run_app(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
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
