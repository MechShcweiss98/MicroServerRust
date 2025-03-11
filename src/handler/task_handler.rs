use crate::{
  model::task_model::{CreateTaskReq, TaskModel, UpdateTaskReq},
  service::task_service::TaskService,
  utils::api_error::APIError,
};
use axum::{Extension, Json, extract::Path, response::IntoResponse};
use sea_orm::DatabaseConnection;

//get by id
#[utoipa::path(
  get,
  path = "/tasks/{id}",
  params(
      ("id" = i32, Path, description = "ID from Task")
  ),
  responses(
      (status = 200, body = TaskModel),
      (status = 404, description = "Task not found")
  )
)]
pub async fn get_task_by_id(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
) -> Result<Json<TaskModel>, APIError> {
  TaskService::find_task_by_id(Extension(db), Path(id)).await
}

//Get
#[utoipa::path(
  get,
  path = "/list",
  responses(
      (status = 200, body = TaskModel),
      (status = 400, description = "Bad request")
  )
)]
pub async fn get_task(
  Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<TaskModel>>, APIError> {
  TaskService::list_task(Extension(db)).await
}
//Created
#[utoipa::path(
  post,
  path = "/insert",
  request_body = CreateTaskReq,
  responses(
      (status = 200, body = TaskModel),
      (status = 400, description = "Bad Request")
  )
)]
pub async fn create_task(
  Extension(db): Extension<DatabaseConnection>,
  Json(task_data): Json<CreateTaskReq>,
) -> Result<impl IntoResponse, APIError> {
  TaskService::save_task(Extension(db), Json(task_data)).await
}
//Update task
#[utoipa::path(
  patch,
  path = "/edit/{id}",
  params(
      ("id" = i32, Path, description = "ID from Task")
  ),
  request_body = UpdateTaskReq,
  responses(
      (status = 200, body = Value),
      (status = 404, description = "Task not found")
  )
)]
pub async fn update_task(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
  Json(task_data): Json<UpdateTaskReq>,
) -> Result<(), APIError> {
  TaskService::edit_task(Extension(db), Path(id), Json(task_data)).await
}
//Delete Task
#[utoipa::path(
  delete,
  path = "/delete/{id}",
  params(
      ("id" = i32, Path, description = "ID from Task")
  ),
  responses(
      (status = 200, body = Value),
      (status = 404, description = "Task not found")
  )
)]
pub async fn delete_task(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
) -> Result<(), APIError> {
  TaskService::remove_task(Extension(db), Path(id)).await
}
