use axum::{Extension, Router};
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

use crate::{router::router_task::initial_router, utils};
use migration::m20220101_000001_create_table::Migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  #[doc = " Vector of migrations in time sequence"]
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(Migration),
    ]
  }
}
//Migrator
async fn run_migration(db: &DatabaseConnection) -> Result<(), DbErr> {
  let schema_manager = SchemaManager::new(db);
  println!("Starting migration...");
  
  if let Err (e) = Migrator::up(db, None).await{
    eprintln!("Error to execute migration: {}", e);
    return Err(sea_orm::DbErr::Custom(e.to_string()));
  }
  println!("Migration completed successfully");
  Ok(())
}

pub async fn server_initial() {
  let conn_str = (*utils::constants::DATABASE_URL).clone();
  let db = Database::connect(conn_str)
    .await
    .expect("Failed to connect to db");

  let app: Router = Router::new()
    .nest("/api/", initial_router(db.clone()))
    .layer(Extension(db));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
