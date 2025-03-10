use crate::handler::task_handler::TaskHandler;

use crate::middleware::global_filter_exception::global_err_handler;
use crate::middleware::verify_time::verify_time_middleware;
use axum::middleware;
use axum::{
  Router,
  routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;

pub fn initial_router(db: DatabaseConnection) -> Router {
  Router::new()
    .route("/list", get(TaskHandler::get_task))
    .route("/insert", post(TaskHandler::create_task))
    .route("/task/:id", get(TaskHandler::get_task_by_id))
    .route("/edit/:id", patch(TaskHandler::update_task))
    .route("/delete/:id", delete(TaskHandler::delete_task))
    .layer(middleware::from_fn_with_state(db, verify_time_middleware))
    .layer(middleware::from_fn(global_err_handler))
}
