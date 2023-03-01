use crate::fs::FooAOut;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use common::web::common_respond_to;

impl Responder for FooAOut {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        common_respond_to(self)
    }
}
