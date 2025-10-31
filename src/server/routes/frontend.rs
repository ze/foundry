use axum::{
  body::Body,
  http::{StatusCode, Uri},
  response::Response,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/frontend/dist/"]
struct Assets;

pub async fn frontend(uri: Uri) -> Result<Response<Body>, StatusCode> {
  let path = uri.path().trim_start_matches('/');
  let path = if path.is_empty() { "index.html" } else { path };

  match Assets::get(path) {
    Some(content) => {
      let body = Body::from(content.data);
      let mime = mime_guess::from_path(path).first_or_octet_stream();

      Ok(
        Response::builder()
          .header("Content-Type", mime.as_ref())
          .body(body)
          .unwrap(),
      )
    }
    None => match Assets::get("index.html") {
      Some(index) => {
        let body = Body::from(index.data);
        Ok(
          Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(body)
            .unwrap(),
        )
      }
      None => Err(StatusCode::NOT_FOUND),
    },
  }
}
