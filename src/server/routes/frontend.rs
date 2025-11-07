use axum::{
  body::Body,
  http::{StatusCode, Uri, header},
  response::Response,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend/dist/"]
struct Assets;

pub async fn frontend(uri: Uri) -> Result<Response<Body>, StatusCode> {
  let path = uri.path().trim_start_matches('/');
  let path = if path.is_empty() { "index.html" } else { path };

  let body = Assets::get(path)
    .or_else(|| Assets::get("index.html"))
    .expect("Embedded index.html missing");
  let body = Body::from(body.data);

  let mime = mime_guess::from_path(path).first_or_octet_stream();
  let response = Response::builder()
    .header(header::CONTENT_TYPE, mime.as_ref())
    .body(body)
    .unwrap();

  Ok(response)
}
