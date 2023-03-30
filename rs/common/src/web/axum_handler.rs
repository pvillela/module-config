use axum::Json;
use futures::{Future, FutureExt};
use std::{pin::Pin, sync::Arc};

pub fn handler_of<S, T, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> Arc<dyn Fn(Json<S>) -> Pin<Box<dyn Future<Output = Json<Json<T>>> + Send>>>
where
    S: serde::Deserialize<'static>,
    T: 'static + Send + Sync,
    Fut: 'static + Future<Output = Json<T>> + Send + Sync,
{
    let hdlr = move |Json(input): Json<S>| {
        let fut = f(input).map(Json).boxed();
        fut
    };
    Arc::new(hdlr)
}

pub fn handler_of1<S, T, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> Arc<dyn Fn(Json<S>) -> Pin<Box<dyn Future<Output = Json<Json<T>>> + Send>>>
where
    S: serde::Deserialize<'static>,
    T: Send + Sync,
    Fut: 'static + Future<Output = Json<T>> + Send + Sync,
{
    let hdlr = move |Json(input): Json<S>| {
        let fut = f(input);
        let fut = async move {
            let res = fut.await;
            Json(res)
        };
        fut.boxed()
    };
    Arc::new(hdlr)
}
