use crate::middleware::global_filter_exception::global_err_handler;
use crate::handler::task_handler::{create_task, get_task, delete_task, get_task_by_id, update_task};
use axum::middleware;
use axum::{
  Router,
  routing::{get, post, delete, patch},
};

pub fn initial_router() -> Router {
  Router::new()
    .route("/list", get(get_task))
    .route("/insert", post(create_task))
    .route("/task/:id", get(get_task_by_id))
    .route("/edit/:id", patch(update_task))
    .route("/delete/:id", delete(delete_task))
    .layer(middleware::from_fn(global_err_handler))
}
