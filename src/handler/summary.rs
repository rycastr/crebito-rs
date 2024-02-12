use axum::{
    body::Body, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::types::chrono::{DateTime, Utc};

use crate::infra::app::AppState;

use super::transaction::TransactionInputType;

#[derive(Serialize, Deserialize)]
struct AccountSummary {
    balance: Option<i64>,
    available_limit: Option<i64>,
    latest_entries: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    amount: i64,
    description: String,
    inserted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct SummaryEntry {
    valor: i64,
    tipo: TransactionInputType,
    descricao: String,
    realizada_em: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct SummaryBalance {
    total: Option<i64>,
    #[serde(rename = "data_extrato")]
    datetime: DateTime<Utc>,
    #[serde(rename = "limite")]
    available_limit: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct SummaryOutput {
    #[serde(rename = "saldo")]
    balance: SummaryBalance,
    #[serde(rename = "ultimas_transacoes")]
    latest_entries: Vec<SummaryEntry>,
}

pub async fn handle(State(app_state): State<AppState>, Path(account_id): Path<i32>) -> Response {
    let result = sqlx::query_as!(
        AccountSummary,
        "SELECT balance, available_limit, latest_entries FROM lookup_statement($1)",
        account_id
    )
    .fetch_optional(&app_state.db_pool_ro)
    .await;

    match result {
        Ok(Some(summary)) => {
            let latest_entries_value = summary.latest_entries.unwrap_or(Value::Array(vec![]));
            match serde_json::from_value::<Vec<Entry>>(latest_entries_value) {
                Ok(latest_entries) => {
                    let summary_output = SummaryOutput {
                        balance: SummaryBalance {
                            total: summary.balance,
                            datetime: Utc::now(),
                            available_limit: summary.available_limit,
                        },
                        latest_entries: latest_entries
                            .into_iter()
                            .map(|entry| SummaryEntry {
                                valor: entry.amount.abs(),
                                tipo: if entry.amount < 0 {
                                    TransactionInputType::Debit
                                } else {
                                    TransactionInputType::Credit
                                },
                                descricao: entry.description,
                                realizada_em: entry.inserted_at,
                            })
                            .collect(),
                    };

                    (StatusCode::OK, Json(json!(summary_output))).into_response()
                }
                Err(_err) => (StatusCode::UNPROCESSABLE_ENTITY, Body::empty()).into_response(),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Body::empty()).into_response(),
        Err(_err) => (StatusCode::UNPROCESSABLE_ENTITY, Body::empty()).into_response(),
    }
}
