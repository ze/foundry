use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub async fn api_not_found() -> (StatusCode, Json<Value>) {
  (
    StatusCode::NOT_FOUND,
    Json(json!({ "message": "not found" })),
  )
}
