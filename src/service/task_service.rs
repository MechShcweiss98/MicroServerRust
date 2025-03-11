use crate::{
  model::task_model::{CreateTaskReq, TaskModel, UpdateTaskReq},
  repository::task_repository::TaskRepository,
  utils::api_error::APIError,
};
use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;

pub struct TaskService;

impl TaskService {
  //get by id
  pub async fn find_task_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
  ) -> Result<Json<TaskModel>, APIError> {
    let task = TaskRepository::find_by_id(&db, id).await?;

    let task_model = TaskModel {
      id: task.id,
      name: task.name,
      description: task.description,
      created_at: task.created_at,
    };
    Ok(Json(task_model))
  }

  //Get
  pub async fn list_task(
    Extension(db): Extension<DatabaseConnection>,
  ) -> Result<Json<Vec<TaskModel>>, APIError> {
    let task = TaskRepository::find_all(&db).await?;

    let task_model: Vec<TaskModel> = task
      .into_iter()
      .map(|item| TaskModel {
        id: item.id,
        name: item.name,
        description: item.description,
        created_at: item.created_at,
      })
      .collect();

    Ok(Json(task_model))
  }
  //Created
  pub async fn save_task(
    Extension(db): Extension<DatabaseConnection>,
    Json(task_data): Json<CreateTaskReq>,
  ) -> Result<impl IntoResponse, APIError> {
    TaskRepository::save(&db, &task_data).await?;
    Ok((StatusCode::ACCEPTED, "Inserted"))
  }
  //Update task
  pub async fn edit_task(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(task_data): Json<UpdateTaskReq>,
  ) -> Result<(), APIError> {
    TaskRepository::update(&db, id, &task_data).await?;
    Ok(())
  }
  //Delete Task
  pub async fn remove_task(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
  ) -> Result<(), APIError> {
    TaskRepository::delete(&db, id).await?;
    Ok(())
  }
}
