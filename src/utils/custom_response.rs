use axum::response::{IntoResponse, Response};
use bytes::{BufMut, BytesMut};
use http::StatusCode;
use serde::Serialize;
use tracing::error;
#[derive(Debug, Clone)]
pub struct CustomResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub body: Option<T>,
}

#[derive(Debug)]
pub struct CustomResponseBuilder<T: Serialize> {
    pub status_code: StatusCode,
    pub body: Option<T>,
}

impl<T> Default for CustomResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status_code: StatusCode::OK,
            body: None,
        }
    }
}

impl<T> CustomResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status_code = status;
        self
    }

    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> CustomResponse<T> {
        CustomResponse {
            status_code: self.status_code,
            body: self.body,
        }
    }
}

impl<T> IntoResponse for CustomResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.body {
            Some(body) => body,
            None => return (self.status_code).into_response(),
        };
        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &body) {
            error!("Error serializing response body as JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        } else {
            let bytes = bytes.into_inner().freeze();
            bytes.into_response()
        }
    }
}
