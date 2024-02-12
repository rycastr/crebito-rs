use std::env;
use std::error;

use crebito::infra::app::{setup_app, AppState};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;

    let host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("APP_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(4000);
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;

    let app_state = AppState::new(pool);
    let app = setup_app(app_state);
    axum::serve(listener, app).await?;

    Ok(())
}
