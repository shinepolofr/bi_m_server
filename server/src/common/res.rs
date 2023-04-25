use std::fmt::Debug;

use axum::{
    body::{self, Full},
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("text/plain; charset=utf-8"),
                    )
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };
        json_string.into_response()
    }
}
