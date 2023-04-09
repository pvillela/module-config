use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use once_cell::sync::OnceCell;

/// Type of boxed and pinned wrapper of async functions.
pub type ArcPinFn<S, T> =
    Arc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync>;

/// Part 1 of the definition of a type alias for a closure that returns a boxed and pinned future.
/// As type aliases for traits are not yet supported, we need to define a new trait and a
/// blanket implementation for it.
/// This is the trait definition.
/// See https://users.rust-lang.org/t/why-cant-type-aliases-be-used-for-traits/10002/9.
pub trait PinFn<S, T>:
    Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync
{
}

// The following type is not valid:
// impl Fn(S) -> (impl Future<Output = T> + 'static + Send + Sync) + Send + Sync
// See https://github.com/rust-lang/rust/issues/99697.

/// Part 2 of the definition of a type alias for a closure that returns a boxed and pinned future.
/// As type aliases for traits are not yet supported, we need to define a new trait and a
/// blanket implementation for it.
/// This is the blanket impl.
/// See https://users.rust-lang.org/t/why-cant-type-aliases-be-used-for-traits/10002/9.
impl<S, T, F> PinFn<S, T> for F where
    F: Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync
{
}

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
pub type RcPinFnWeb<S, T> = Rc<dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static>>>;

/// Boxes and pins an async function so it can be passed across theads.
pub fn arc_pin_async_fn_web<S, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> RcPinFnWeb<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Rc::new(move |s| Box::pin(f(s)))
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

pub fn get_from_once_cell<T>(cell: &OnceCell<T>) -> &T {
    cell.get().expect("OnceCell not initialized.")
}

/// Sets a OnceCell and prints a message if the cell was already initialized.
/// Handling the result if optional if the caller doesn't want to take action in
/// case the cell was already initialized.
pub fn set_once_cell<T>(cell: &OnceCell<T>, x: T) -> Result<(), T> {
    let res = cell.set(x);
    if res.is_err() {
        println!("OnceCell already initialized.");
    }
    res
}

/// Returns a static reference to a value.
/// The reference is required to be static to avoid memory leaks.
pub fn static_ref<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
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

pub fn compose_static_0_arc<S: 'static, T>(
    f: impl Fn() -> Arc<S> + 'static + Send + Sync, //fn() -> Arc<S>,
    g: impl Fn(&S) -> T + 'static + Send + Sync,    //fn(&S) -> T,
) -> &'static (dyn Fn() -> T + Send + Sync) {
    Box::leak(Box::new(move || g(&f())))
}

pub fn static_closure_0_thread_safe<T>(
    f: impl Fn() -> T + Send + Sync + 'static,
) -> &'static (dyn Fn() -> T + Send + Sync) {
    Box::leak(Box::new(f))
}

pub type StaticFn<S, T> = &'static (dyn Fn(S) -> T + Send + Sync);

pub type StaticFn0<T> = &'static (dyn Fn() -> T + Send + Sync);
