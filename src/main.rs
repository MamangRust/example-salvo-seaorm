use dotenv::dotenv;

use example_salvo_seaorm::config::{Config, ConnectionManager};
use example_salvo_seaorm::handler::AppRouter;
use example_salvo_seaorm::migrations::m20220101_000001_create_table::Migration;
use example_salvo_seaorm::state::AppState;
use example_salvo_seaorm::utils::tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing();

    let config = Config::init();

    let db_pool =
        ConnectionManager::new_pool::<Migration>(&config.database_url, config.run_migrations)
            .await?;

    let port = config.port;

    let state = AppState::new(db_pool, &config.jwt_secret);

    println!("🚀 Server started successfully");

    AppRouter::serve(port, state).await
}