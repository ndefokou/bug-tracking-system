use axum::{Router};
use crate::handlers::bug_handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/bugs", get(bug_handlers::list_bugs))
        .route("/bugs", post(bug_handlers::create_bug))
}
