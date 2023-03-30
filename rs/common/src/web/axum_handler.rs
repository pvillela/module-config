use crate::fwk::{arc_pin_async_fn, ArcPinFn};
use axum::Json;
use futures::Future;
use std::{pin::Pin, sync::Arc};

pub fn handler_of<S, T>(
    f: ArcPinFn<S, T>,
) -> Arc<dyn Fn(Json<S>) -> Pin<Box<dyn Future<Output = Json<T>> + Send + Sync>> + Send + Sync>
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
        fut
    };
    arc_pin_async_fn(hdlr)
}
