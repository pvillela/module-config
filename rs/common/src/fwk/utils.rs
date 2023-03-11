use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type of boxed and pinned wrapper of async functions.
pub type BoxPinFn<S, T> =
    Arc<dyn FnMut(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn box_pin_async_fn<S: 'static, T: Send + Sync, Fut>(
    mut f: impl FnMut(S) -> Fut + 'static + Send + Sync,
) -> BoxPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Arc::new(move |s| Box::pin(f(s)))
}

pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
