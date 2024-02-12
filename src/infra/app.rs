use axum::{routing::{get, post}, Router};
use sqlx::{Pool, Postgres};

use crate::handler::{healthcheck, summary, transaction};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(db_pool: Pool<Postgres>) -> AppState {
        AppState { db_pool }
    }
}

pub fn setup_app(app_state: AppState) -> Router {
    Router::new()
        .route("/clientes/:account_id/transacoes", post(transaction::handle))
        .route("/clientes/:account_id/extrato", get(summary::handle))
        .with_state(app_state)
        .route("/healthcheck", get(healthcheck::handle))
}
