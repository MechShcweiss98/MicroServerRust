use crate::{
  model::task_model::{CreateTaskReq, UpdateTaskReq},
  utils::api_error::{APIError, handle_db_error},
};
use axum::http::StatusCode;
use chrono::Utc;
use entity::task;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

async fn find_task_by_id(db: &DatabaseConnection, id: i32) -> Result<task::Model, APIError> {
  task::Entity::find_by_id(id)
    .one(db)
    .await
    .map_err(handle_db_error)?
    .ok_or(APIError {
      message: "Task not found".to_owned(),
      status_code: StatusCode::NOT_FOUND,
      error_code: Some(50),
    })
}
pub struct TaskRepository;

impl TaskRepository {
  //Fin by Value
  pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<task::Model, APIError> {
    task::Entity::find_by_id(id)
      .one(db)
      .await
      .map_err(handle_db_error)?
      .ok_or(APIError {
        message: "Task not found".to_owned(),
        status_code: StatusCode::NOT_FOUND,
        error_code: Some(50),
      })
  }
  // take all register
  pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<task::Model>, APIError> {
    task::Entity::find().all(db).await.map_err(handle_db_error)
  }
  // build a task
  pub async fn save(
    db: &DatabaseConnection,
    data: &CreateTaskReq,
  ) -> Result<task::Model, APIError> {
    let task = task::ActiveModel {
      name: Set(data.name.to_owned()),
      description: Set(data.description.to_owned()),
      created_at: Set(Utc::now().naive_utc()),
      ..Default::default()
    };

    task.insert(db).await.map_err(handle_db_error)
  }

  //update one register
  pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    data: &UpdateTaskReq,
  ) -> Result<(), APIError> {
    let mut task: entity::task::ActiveModel = find_task_by_id(&db, id).await?.into();

    task.name = Set(data.name.clone().unwrap_or(task.name.take().unwrap()));
    task.description = Set(
      data
        .description
        .clone()
        .unwrap_or(task.description.take().unwrap()),
    );

    task.update(db).await.map_err(handle_db_error)?;
    Ok(())
  }
  //delete one register
  pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), APIError> {
    let task = find_task_by_id(&db, id).await?;

    task::Entity::delete_by_id(task.id)
      .exec(db)
      .await
      .map_err(handle_db_error)?;

    Ok(())
  }
}
