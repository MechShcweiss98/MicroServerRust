use axum::{
  body::Body,
  http::{Request, StatusCode},
  middleware::Next,
  response::{IntoResponse, Response},
};

use crate::utils::api_error::APIError;

pub async fn global_err_handler<B>(req: Request<B>, next: Next) -> Response
where
  B: Send + 'static,
  Body: From<B>,
{
  let req = req.map(Body::from);

  let response = next.run(req).await;

  if response.status().is_server_error() {
    let error_response = APIError {
      message: "Internal Error Server".to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error_code: Some(50),
    };
    return error_response.into_response();
  }
  response
}
