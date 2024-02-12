use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::infra::app::AppState;

#[derive(Serialize, Deserialize)]
pub enum TransactionInputType {
    #[serde(rename = "c")]
    Credit,
    #[serde(rename = "d")]
    Debit,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionInput {
    valor: i64,
    descricao: String,
    tipo: TransactionInputType,
}

#[derive(Serialize, Deserialize)]
struct TransactionOutput {
    #[serde(rename = "saldo")]
    new_balance: Option<i64>,
    #[serde(rename = "limite")]
    available_limit: Option<i64>,
}

pub async fn handle(
    State(app_state): State<AppState>,
    Path(account_id): Path<i32>,
    Json(tx_input): Json<TransactionInput>,
) -> Response {
    if tx_input.descricao.len() > 10 || tx_input.descricao.len() < 1 {
        return (StatusCode::UNPROCESSABLE_ENTITY, Body::empty()).into_response();
    }

    let result = sqlx::query_as!(
        TransactionOutput,
        "SELECT new_balance, available_limit FROM perform_transaction($1, $2, $3, $4)",
        Uuid::new_v4(),
        account_id,
        match tx_input.tipo {
            TransactionInputType::Credit => tx_input.valor,
            TransactionInputType::Debit => -tx_input.valor,
        },
        tx_input.descricao
    )
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(tx_output) => (StatusCode::OK, Json(json!(tx_output))).into_response(),
        Err(err) => match err {
            sqlx::Error::Database(e) => match e.code().unwrap_or_default().as_ref() {
                "TRX01" => (StatusCode::NOT_FOUND, Body::empty()).into_response(),
                "TRX02" => (StatusCode::UNPROCESSABLE_ENTITY, Body::empty()).into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Body::empty()).into_response(),
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Body::empty()).into_response(),
        },
    }
}
