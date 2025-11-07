use std::sync::Arc;

use axum::{
  Router,
  routing::{any, get},
};

use crate::{
  font::project::Project,
  server::{
    routes::{
      api::{api_get_config, api_get_font, api_not_found},
      frontend::frontend,
    },
    state::ApiState,
  },
};

pub fn router(project: Arc<Project>) -> Router {
  let state = ApiState::new(project);

  let api = Router::new()
    .route("/config", get(api_get_config))
    .route("/font", get(api_get_font))
    .fallback(api_not_found)
    .with_state(state);

  Router::new()
    .nest("/api", api)
    .route("/api/", any(api_not_found))
    .fallback(frontend)
}
