use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn handle() -> Response {
    (StatusCode::OK, Json(json!({"ok": true}))).into_response()
}
