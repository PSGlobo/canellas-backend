mod log;

use ps_globo::{app::App, config::Settings};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let subscriber = log::get_subscriber("info".into());
    log::init_subscriber(subscriber);
    info!("Log started");

    let settings = Settings::load().expect("Load settings");
    info!("Settings loaded");

    let app = App::build(&settings).await.expect("couldn't build the app");
    app.run().await
}
