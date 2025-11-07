use std::fs;

use axum::{
  Json,
  body::Body,
  extract::State,
  http::{HeaderValue, Response, StatusCode, header},
};
use serde_json::{Value, json};

use crate::server::state::ApiState;

pub async fn api_get_font(State(state): State<ApiState>) -> Response<Body> {
  let font = state.project().font_path();
  let font_name = font.file_name().unwrap().to_str().unwrap();
  let font = fs::read(&font).unwrap();
  let body = Body::from(font);

  Response::builder()
    .header(header::CONTENT_TYPE, HeaderValue::from_static("font/ttf"))
    .header(
      header::CONTENT_DISPOSITION,
      format!("inline; filename=\"{font_name}\""),
    )
    .body(body)
    .unwrap()
}

pub async fn api_get_config(State(state): State<ApiState>) -> Json<Value> {
  let json = serde_json::to_value(state.project().config()).unwrap();
  Json(json)
}

pub async fn api_not_found() -> (StatusCode, Json<Value>) {
  (
    StatusCode::NOT_FOUND,
    Json(json!({ "message": "not found" })),
  )
}
