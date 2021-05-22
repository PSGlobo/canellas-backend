mod log;

use std::net::TcpListener;

use ps_globo::{app::run_app, config::Settings, db::create_pool};
use tracing::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let subscriber = log::get_subscriber("info".into());
    log::init_subscriber(subscriber);
    info!("Log started");

    let settings = Settings::load().expect("Load settings");
    info!("Settings loaded");

    let pool = create_pool(&settings.database).await;
    info!("Database connected");
    debug!("{:?}", &pool);

    let listener = TcpListener::bind((settings.host, settings.port))?;
    let addr = listener.local_addr().unwrap();
    info!(
        "Server's connection listener set up on {}:{}",
        addr.ip(),
        addr.port()
    );

    run_app(listener, pool)?.await
}
