//! App Startup

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{get, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{db_test, hello};

/// Creates, configures and starts an instance of the server.
pub fn run_app(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/", get().to(hello))
            .route("/db_test", get().to(db_test))
            .route("/{name}", get().to(hello))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
