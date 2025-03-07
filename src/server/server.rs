use axum::Router;

use crate::router::router_task::initial_router;

pub async fn server_initial() {
  let app: Router = Router::new().nest("/api/", initial_router());

  let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}