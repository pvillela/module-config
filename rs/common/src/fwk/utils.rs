use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

/// Type of boxed and pinned wrapper of async functions.
pub type ArcPinFn<S, T> =
    Arc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn arc_pin_async_fn<S: 'static, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> ArcPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Arc::new(move |s| Box::pin(f(s)))
}

/// Type of boxed and pinned wrapper of async functions.
pub type ArcPinFnWeb<S, T> = Arc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static>>>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn arc_pin_async_fn_web<S: 'static, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> ArcPinFnWeb<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Arc::new(move |s| Box::pin(f(s)))
}

pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Type of boxed and pinned wrapper of async functions.
pub type RcPinFn<S, T> = Rc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static>>>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn rc_pin_async_fn<S: 'static, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> RcPinFn<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Rc::new(move |s| Box::pin(f(s)))
}
