use axum::{Extension, Router};
use sea_orm::Database;

// use crate::utils::swagger_conf::ApiDoc;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

use crate::{router::router_task::initial_router, utils};

pub async fn server_initial() {

  // let api = ApiDoc::openapi();
  // let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api);

  let conn_str = (*utils::constants::DATABASE_URL).clone();
  let db = Database::connect(conn_str)
    .await
    .expect("Failed to connect to db");

  let app: Router = Router::new()
    .nest("/api/", initial_router(db.clone()))
    // .merge(swagger_ui)
    .layer(Extension(db));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
