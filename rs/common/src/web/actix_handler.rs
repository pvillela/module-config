use crate::fwk::{BoxPinFn, BoxPinFnWss, PinFn, RcPinFnWss};
use actix_web::{body::BoxBody, http::header::ContentType, web::Json, HttpResponse, Responder};
use futures::Future;
use std::{pin::Pin, rc::Rc, sync::Arc};

pub fn common_respond_to<T: serde::Serialize>(t: T) -> HttpResponse<BoxBody> {
    let body = serde_json::to_string(&t).unwrap();

    // Create response and set content type
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

pub fn handler_arc_of<S: 'static + serde::Deserialize<'static>, T: Responder + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> Arc<dyn Fn(Json<S>) -> Fut + Send + Sync + 'static>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Arc::new(move |Json(input)| f(input))
}

pub fn handler_of<S: 'static + serde::Deserialize<'static>, T: Responder + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync + Clone,
) -> impl Fn(Json<S>) -> Fut + Send + Sync + 'static + Clone
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    move |Json(input)| f(input)
}

pub fn handler_of_wss<S: 'static + serde::Deserialize<'static>, T: Responder + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync + Clone,
) -> impl Fn(Json<S>) -> Fut + Send + Sync + 'static + Clone
where
    Fut: Future<Output = T>,
{
    move |Json(input)| f(input)
}

pub fn handler_arc_of_rcpin_wss<
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Responder + Send + Sync,
>(
    f: RcPinFnWss<S, T>,
) -> Arc<dyn Fn(Json<S>) -> Pin<Box<dyn Future<Output = T>>> + 'static> {
    Arc::new(move |Json(input)| f(input))
}

pub fn handler_of_rcpin_wss<
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Responder + Send + Sync,
>(
    f: RcPinFnWss<S, T>,
) -> impl Fn(Json<S>) -> Pin<Box<dyn Future<Output = T>>> + 'static + Clone {
    move |Json(input)| f(input)
}

pub fn handler_of_boxpin_wss<
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Responder + Send + Sync,
>(
    f: BoxPinFnWss<S, T>,
) -> impl Fn(Json<S>) -> Pin<Box<dyn Future<Output = T>>> + 'static + Clone {
    let f: RcPinFnWss<S, T> = Rc::from(f);
    move |Json(input)| f(input)
}

pub fn handler_of_rcpin<
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Responder + Send + Sync,
>(
    f: Rc<PinFn<S, T>>,
) -> impl Fn(Json<S>) -> Pin<Box<dyn Future<Output = T>>> + 'static + Clone {
    move |Json(input)| f(input)
}

pub fn handler_of_boxpin<
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Responder + Send + Sync,
>(
    f: BoxPinFn<S, T>,
) -> impl Fn(Json<S>) -> Pin<Box<dyn Future<Output = T>>> + 'static + Clone {
    let f: Rc<PinFn<S, T>> = Rc::from(f);
    move |Json(input)| f(input)
}
