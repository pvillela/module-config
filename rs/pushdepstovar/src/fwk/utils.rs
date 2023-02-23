use std::pin::Pin;
use std::future::Future;

/// Boxes and pins an async function so it can be passed across theads.
pub fn box_pin_async_fn<S: 'static, T: Send + Sync, F>(
    f: fn(S) -> F,
) -> Box<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + Send + Sync>> + Send + Sync>
where
    F: 'static + Future<Output = T> + Send + Sync,
{
    Box::new(move |s| Box::pin(f(s)))
}

pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
