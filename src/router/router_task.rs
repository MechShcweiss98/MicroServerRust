use crate::handler::task_handler::{
  create_task, delete_task, get_task, get_task_by_id, update_task,
};

use crate::middleware::global_filter_exception::global_err_handler;
use crate::middleware::verify_time::verify_time_middleware;

// use crate::utils::swagger_conf::ApiDoc;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

use axum::middleware;
use axum::{
  Router,
  routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;

pub fn initial_router(db: DatabaseConnection) -> Router {
  // let api = ApiDoc::openapi();
  // let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api);

  Router::new()
    .route("/list", get(get_task))
    .route("/insert", post(create_task))
    .route("/task/:id", get(get_task_by_id))
    .route("/edit/:id", patch(update_task))
    .route("/delete/:id", delete(delete_task))
    .layer(middleware::from_fn_with_state(db, verify_time_middleware))
    .layer(middleware::from_fn(global_err_handler))
    // .merge(swagger_ui)
}
