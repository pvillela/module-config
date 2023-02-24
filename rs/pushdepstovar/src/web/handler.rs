use crate::fs::{foo_a_sfl, FooAIn, FooAOut};
use actix_web::{
    body::BoxBody, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};

impl Responder for FooAOut {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub async fn foo_handler(info: web::Json<FooAIn>) -> FooAOut {
    let input = info.into_inner();
    foo_a_sfl(input).await
}
