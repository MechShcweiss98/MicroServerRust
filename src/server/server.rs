use axum::{Extension, Router};
use sea_orm::Database;

use crate::{router::router_task::initial_router, utils};

pub async fn server_initial() {
  let conn_str = (*utils::constants::DATABASE_URL).clone();
  let db = Database::connect(conn_str)
    .await
    .expect("Failed to connect to db");

  let app: Router = Router::new()
    .nest("/api/", initial_router())
    .layer(Extension(db));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
