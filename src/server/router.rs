use std::sync::Arc;

use axum::{Router, routing::any};

use crate::{
  font::project::Project,
  server::{
    routes::{api::api_not_found, frontend::frontend},
    state::RouterState,
  },
};

pub fn router(project: Project) -> Router {
  let state = Arc::new(RouterState::new(project));

  let api = Router::new().fallback(api_not_found).with_state(state);

  Router::new()
    .nest("/api", api)
    .route("/api/", any(api_not_found))
    .fallback(frontend)
}
