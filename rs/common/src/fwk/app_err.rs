use super::DbErr;
use crate::web::actix_handler::common_respond_to;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder, ResponseError};
use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use serde::Serialize;

/// type of application errors.
#[derive(Serialize, Debug, Display, Error)]
pub struct AppErr;

impl From<DbErr> for AppErr {
    fn from(_db_err: DbErr) -> Self {
        // TODO: properly implement this
        AppErr
    }
}

impl Responder for AppErr {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        common_respond_to(self)
    }
}

impl IntoResponse for AppErr {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

/// Default implementation.
/// See https://actix.rs/docs/errors/.
impl ResponseError for AppErr {}
