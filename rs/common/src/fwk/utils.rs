use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use std::sync::OnceLock;

/// Type of dynamic object of pinned wrapper of async closures.
pub type PinFn<S, T> =
    dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync;

/// Type of Arced and pinned wrapper of async closures.
pub type ArcPinFn<S, T> = Arc<PinFn<S, T>>;

/// Type of boxed and pinned wrapper of async closures.
pub type BoxPinFn<S, T> = Box<PinFn<S, T>>;

/// Type of static reference to desugared async closure.
pub type RefPinFn<S, T> = &'static PinFn<S, T>;

/// Type of dynamic object of pinned wrapper of async closures, without Send + Sync..
pub type PinFnWeb<S, T> = dyn Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static>>;

/// Type of Rc'd and pinned wrapper of async functions, without Send + Sync.
pub type RcPinFnWeb<S, T> = Rc<PinFnWeb<S, T>>;

/// Type of boxed and pinned wrapper of async functions, without Send + Sync.
pub type BoxPinFnWeb<S, T> = Box<PinFnWeb<S, T>>;

/// Part 1 of the definition of a type alias for a closure that returns a boxed and pinned future.
/// As type aliases for traits are not yet supported, we need to define a new trait and a
/// blanket implementation for it.
/// This is the trait definition.
/// See https://users.rust-lang.org/t/why-cant-type-aliases-be-used-for-traits/10002/9.
pub trait PinFnExperimental<S, T>:
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
impl<S, T, F> PinFnExperimental<S, T> for F where
    F: Fn(S) -> Pin<Box<dyn Future<Output = T> + 'static + Send + Sync>> + Send + Sync
{
}

/// Desugared type of async function with single argument and boxed and pinned output.
pub type Pinfn<S, T> = fn(S) -> Pin<Box<dyn Future<Output = T> + Send + Sync>>;

/// Desugared type of async function with two arguments and boxed and pinned output.
pub type Pinfn2<S1, S2, T> = fn(S1, S2) -> Pin<Box<dyn Future<Output = T> + Send + Sync>>;

/// Arcs and pins an async function so it can be passed as a param or return type across theads.
pub fn arc_pin_async_fn<S: 'static, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> ArcPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Arc::new(move |s| Box::pin(f(s)))
}

/// Boxes and pins an async function so it can be passed as a param or return type.
pub fn box_pin_async_fn<S: 'static, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> BoxPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Box::new(move |s| Box::pin(f(s)))
}

/// Transforms an async closure into a closure that returns a pinned-boxed future.
pub fn pin_async_fn<S: 'static, T: 'static + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> impl Fn(S) -> Pin<Box<dyn 'static + Future<Output = T> + Send + Sync>>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    move |s| {
        let x = f(s);
        let y: Pin<Box<dyn 'static + Future<Output = T> + Send + Sync>> = Box::pin(x);
        y
    }
}

/// Transforms an async closure into a static (leaked) reference to a closure that returns a pinned-boxed future.
/// Same functionality as [ref_pin_async_fn] with different implementation details.
pub fn ref_pin_async_fn_original<S, T: 'static + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> RefPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Box::leak(Box::new(pin_async_fn(f)))
}

/// Transforms an async closure into a static (leaked) reference to a closure that returns a pinned-boxed future.
pub fn ref_pin_async_fn<S, T: 'static + Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static + Send + Sync,
) -> RefPinFn<S, T>
where
    Fut: 'static + Future<Output = T> + Send + Sync,
{
    Box::leak(box_pin_async_fn(f))
}

/// Rc's and pins an async function so it can be passed as a param or return type.
pub fn rc_pin_async_fn_web<S, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> RcPinFnWeb<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Rc::new(move |s| Box::pin(f(s)))
}

/// Boxed and pins an async function so it can be passed as a param or return type.
pub fn box_pin_async_fn_web<S, T: Send + Sync, Fut>(
    f: impl Fn(S) -> Fut + 'static,
) -> BoxPinFnWeb<S, T>
where
    Fut: 'static + Future<Output = T>,
{
    Box::new(move |s| Box::pin(f(s)))
}

/// Type of minimalistic boxed wrapper of async functions.
pub type MinBoxFn<S, T> = Box<dyn Fn(S) -> Box<dyn Future<Output = T>>>;

/// Boxes and pins an async function so it can be passed.
pub fn min_box_async_fn<S, T, Fut>(f: impl Fn(S) -> Fut + 'static) -> MinBoxFn<S, T>
where
    Fut: Future<Output = T> + 'static,
{
    Box::new(move |s| Box::new(f(s)))
}

pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Function that can be used as a placeholder for a configuration source during development.
/// Supports any configuration info type and panics if called.
pub fn nil_app_cfg<T>() -> Arc<T> {
    todo!("Configuration source not provided.")
}

pub fn get_from_once_lock<T>(cell: &OnceLock<T>) -> &T {
    cell.get().expect("OnceLock not initialized.")
}

/// Sets a OnceLock and prints a message if the cell was already initialized.
/// Handling the result if optional if the caller doesn't want to take action in
/// case the cell was already initialized.
pub fn set_once_lock<T>(cell: &OnceLock<T>, x: T) -> Result<(), T> {
    let res = cell.set(x);
    if res.is_err() {
        println!("OnceLock already initialized.");
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

pub fn static_closure_1_thread_safe<S, T>(
    f: impl Fn(S) -> T + Send + Sync + 'static,
) -> &'static (dyn Fn(S) -> T + Send + Sync) {
    Box::leak(Box::new(f))
}

pub type StaticFn<S, T> = &'static (dyn Fn(S) -> T + Send + Sync);

pub type StaticFn0<T> = &'static (dyn Fn() -> T + Send + Sync);

/// Gets the value from an Option<T> and returns a reference to T.
/// Panics if the source value is None.
/// This function is used to retrieve values from static mutable variables.
pub fn get_initialized_option<T: Sync>(info_src: &Option<T>) -> &T {
    info_src.as_ref().expect("Option not initialized")
}

/// Initializes value if it is None, no-op otherwise.
/// This function is used to set the values of static mutable variables.
/// It should only be called by the main thread during application initialization, before
/// any access to the variable.
pub fn init_option<T: Sync>(target: &mut Option<T>, info: T) {
    if target.is_none() {
        *target = Some(info);
    }
}
