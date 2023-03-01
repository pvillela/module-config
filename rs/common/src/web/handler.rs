use actix_web::{body::BoxBody, http::header::ContentType, web, HttpResponse, Responder};
use futures::Future;
use std::sync::Arc;

pub fn common_respond_to<T: serde::Serialize>(t: T) -> HttpResponse<BoxBody> {
    let body = serde_json::to_string(&t).unwrap();

    // Create response and set content type
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

pub fn handler_of<S: 'static + serde::Deserialize<'static>, T: Responder + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> Arc<dyn Fn(web::Json<S>) -> Fut + Send + Sync + 'static>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Arc::new(move |info: web::Json<S>| {
        let input = info.into_inner();
        f(input)
    })
}
