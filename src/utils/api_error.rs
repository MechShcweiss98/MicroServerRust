use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
  pub message: String,
  pub status_code: StatusCode,
  pub error_code: Option<i8>,
}

impl IntoResponse for APIError {
  fn into_response(self) -> axum::response::Response {
    let status_code = self.status_code;
    (
      status_code,
      Json(json!({
        "status":"error",
        "statusCode":status_code.as_u16(),
        "errorCode":self.error_code,
        "message":self.message
      })),
    )
      .into_response()
  }
}

pub fn handle_db_error<E: std::fmt::Display>(e: E) -> APIError {
  APIError {
    message: e.to_string(),
    status_code: StatusCode::INTERNAL_SERVER_ERROR,
    error_code: Some(50),
  }
}
