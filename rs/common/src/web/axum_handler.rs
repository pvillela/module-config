use crate::fwk::{ArcPinFn, BoxPinFn};
use axum::response::IntoResponse;
use axum::Json;
use futures::{Future, FutureExt};
use std::{pin::Pin, sync::Arc};

pub fn handler_of_arcpin<S, T>(
    f: ArcPinFn<S, T>,
) -> impl Fn(Json<S>) -> Pin<Box<(dyn Future<Output = Json<T>> + Send + 'static)>>
       + Send
       + Sync
       + 'static
       + Clone
where
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Send + Sync,
{
    let hdlr = move |Json(input): Json<S>| {
        let fut = f(input);
        let fut = async move {
            let res = fut.await;
            Json(res)
        };
        fut.boxed()
    };
    hdlr
}

pub fn handler_of_pin<S, T>(
    f: impl Fn(S) -> Pin<Box<dyn Future<Output = T> + Send + Sync>> + Send + Sync + 'static,
) -> impl Fn(Json<S>) -> Pin<Box<(dyn Future<Output = Json<T>> + Send + 'static)>>
       + Send
       + Sync
       + 'static
       + Clone
where
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Send + Sync,
{
    let f: ArcPinFn<S, T> = Arc::new(f);
    let hdlr = move |Json(input): Json<S>| {
        let fut = f(input);
        let fut = async move {
            let res = fut.await;
            Json(res)
        };
        fut.boxed()
    };
    hdlr
}

pub fn handler_of_boxpin<S, T>(
    f: BoxPinFn<S, T>,
) -> impl Fn(Json<S>) -> Pin<Box<(dyn Future<Output = Json<T>> + Send + 'static)>>
       + Send
       + Sync
       + 'static
       + Clone
where
    S: 'static + serde::Deserialize<'static>,
    T: 'static + Send + Sync,
{
    let f: ArcPinFn<S, T> = Arc::from(f);
    let hdlr = move |Json(input): Json<S>| {
        let fut = f(input);
        let fut = async move {
            let res = fut.await;
            Json(res)
        };
        fut.boxed()
    };
    hdlr
}

pub fn handler_of<S, T, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync + Clone,
) -> impl Fn(Json<S>) -> Fut + Send + Sync + 'static + Clone
where
    S: 'static + serde::Deserialize<'static>,
    T: IntoResponse + Send + Sync,
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    move |Json(input)| f(input)
}
