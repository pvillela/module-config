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

// /// Type of boxed and pinned wrapper of async functions.
// pub type ArcPinFnL<'a, S, T> =
//     Box<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'a + Send + Sync>> + Send + Sync>;

// /// Boxes and pins an async function so it can be passed across theads.
// pub fn arc_pin_async_fn_l<'a, S: 'a, T: Send + Sync, Fut>(
//     f: impl Fn(S) -> Fut + 'a + Send + Sync,
// ) -> ArcPinFnL<'a, S, T>
// where
//     Fut: 'a + Future<Output = T> + Send + Sync,
// {
//     Box::new(move |s| Box::pin(f(s)))
//     // todo!()
// }

/// Type of boxed and pinned wrapper of async functions.
pub type ArcPinFnWeb<S, T> = Arc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static>>>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn arc_pin_async_fn_web<S, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> ArcPinFnWeb<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Arc::new(move |s| Box::pin(f(s)))
}

/// Type of boxed wrapper of async functions.
pub type MinBoxFn<S, T> = Box<dyn Fn(S) -> Box<dyn Future<Output = T>>>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn min_box_async_fn<S, T, Fut>(f: impl Fn(S) -> Fut + 'static) -> MinBoxFn<S, T>
where
    Fut: Future<Output = T> + 'static,
{
    Box::new(move |s| Box::new(f(s)))
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

/// Function that can be used as a placeholder for a configuration source during development.
/// Supports any configuration info type and panics if called.
pub fn nil_app_cfg<T>() -> Arc<T> {
    todo!("Configuration source not provided.")
}

/// Returns a static reference to a value or an override if the override exists.
/// The references are required to be static to avoid memory leaks.
pub fn static_ref_with_override<T>(ovd: Option<&'static T>, value: T) -> &'static T {
    if let Some(ovd) = ovd {
        ovd
    } else {
        Box::leak(Box::new(value))
    }
}
