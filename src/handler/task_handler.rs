use crate::{
  model::task_model::{CreateTaskReq, TaskModel, UpdateTaskReq},
  utils::api_error::APIError,
};
use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use entity::task;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

//get by id
pub async fn get_task_by_id(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
) -> Result<Json<TaskModel>, APIError> {
  let task = entity::task::Entity::find_by_id(id)
    .one(&db)
    .await
    .map_err(|e| APIError {
      message: e.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    })?
    .ok_or(APIError {
      message: "Task not found".to_owned(),
      status_code: StatusCode::NOT_FOUND,
      error_code: Some(50),
    })?;

  let task_model = TaskModel {
    id: task.id,
    name: task.name,
    description: task.description,
    created_at: task.created_at,
  };
  Ok(Json(task_model))
}

//Get
pub async fn get_task(
  Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<TaskModel>>, APIError> {
  let task: Vec<TaskModel> = entity::task::Entity::find()
    .all(&db)
    .await
    .map_err(|err| APIError {
      message: err.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    })?
    .into_iter()
    .map(|item| TaskModel {
      id: item.id,
      name: item.name,
      description: item.description,
      created_at: item.created_at,
    })
    .collect();

  Ok(Json(task))
}
//Created
pub async fn create_task(
  Extension(db): Extension<DatabaseConnection>,
  Json(task_data): Json<CreateTaskReq>,
) -> impl IntoResponse {
  let task_model = task::ActiveModel {
    name: Set(task_data.name.to_owned()),
    description: Set(task_data.description.to_owned()),
    created_at: Set(Utc::now().naive_utc()),
    ..Default::default()
  };

  task_model.insert(&db).await.unwrap();

  // db.close().await.unwrap();
  (StatusCode::ACCEPTED, "Inserted")
}
//Update task
pub async fn update_task(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
  Json(task_data): Json<UpdateTaskReq>,
) -> Result<(), APIError> {
  let mut task: entity::task::ActiveModel = entity::task::Entity::find()
    .filter(entity::task::Column::Id.eq(id))
    .one(&db)
    .await
    .map_err(|e| APIError {
      message: e.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    })?
    .ok_or(APIError {
      message: "Task not found".to_owned(),
      status_code: StatusCode::NOT_FOUND,
      error_code: Some(50),
    })?
    .into();

  task.name = Set(task_data.name.unwrap_or(task.name.take().unwrap()));
  task.description = Set(
    task_data
      .description
      .unwrap_or(task.description.take().unwrap()),
  );

  task.update(&db).await.map_err(|e| APIError {
    status_code: StatusCode::INTERNAL_SERVER_ERROR,
    message: e.to_string(),
    error_code: Some(50),
  })?;

  Ok(())
}
//Delete Task
pub async fn delete_task(
  Extension(db): Extension<DatabaseConnection>,
  Path(id): Path<i32>,
) -> Result<(), APIError> {
  let task = entity::task::Entity::find()
    .filter(entity::task::Column::Id.eq(id))
    .one(&db)
    .await
    .map_err(|e| APIError {
      message: e.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    })?
    .ok_or(APIError {
      message: "Not Found".to_owned(),
      status_code: StatusCode::NOT_FOUND,
      error_code: Some(50),
    })?;

  entity::task::Entity::delete_by_id(task.id)
    .exec(&db)
    .await
    .map_err(|e| APIError {
      message: e.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    })?;

  Ok(())
}
