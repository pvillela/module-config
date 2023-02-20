use arc_swap::ArcSwap;
use std::{borrow::Borrow, ops::Deref, sync::Arc};

/// Wrapped type for a static atomic mutable variable that holds a higher-order function
/// that transforms fn() -> Arc<S> to Fn() -> Arc<T>
pub type StaticCfgAdapter<S, T> =
    ArcSwap<Box<dyn Send + Sync + Fn(fn() -> Arc<S>) -> Box<dyn Send + Sync + Fn() -> Arc<T>>>>;

/// Wrapped type for a higher-order function that transforms fn() -> Arc<S> to Fn() -> Arc<T>
type ArcedCfgAdapter<S, T> =
    Arc<Box<dyn Send + Sync + Fn(fn() -> Arc<S>) -> Box<dyn Send + Sync + Fn() -> Arc<T>>>>;

/// Higher-order function that transforms a fn(&S) -> T to a StaticCfgAdapter<S, T>.
/// Used to initiaize static config adapters.
pub fn lift_to_nullary<S: 'static, T: 'static>(f: fn(&S) -> T) -> StaticCfgAdapter<S, T> {
    ArcSwap::new(lift_to_nullary0(f))
}

/// Higher-order function that transforms a fn(&S) -> T to an ArcedCfgAdapter<S, T>
fn lift_to_nullary0<S: 'static, T: 'static>(f: fn(&S) -> T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |s_src: fn() -> Arc<S>| {
        Box::new(move || Arc::new(f(&s_src())))
    }))
}

/// Higher-order function that transforms a fn() -> T to an ArcedCfgAdapter<S, T>
fn lift_to_nullary1<S, T: 'static>(f: fn() -> T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |_s_src: fn() -> Arc<S>| {
        Box::new(move || Arc::new(f()))
    }))
}

/// Higher-order function that transforms a T to an ArcedCfgAdapter<S, T>
fn lift_to_nullary2<S, T: 'static + Clone + Send + Sync>(x: T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |_s_src: fn() -> Arc<S>| {
        let y = x.clone();
        Box::new(move || Arc::new(y.clone()))
    }))
}

/// Function that can be used as a placeholder for a configuration source during development.
/// Supports any configuration info type and panics if called.
pub fn nil_app_cfg<T>() -> Arc<T> {
    todo!("Configuration source not provided.")
}

/// Updates a StaticCfgAdapter<S, T> with the passed in function, lifting it to a nullary.
/// The passed in function transforms the output of the source application configuration source.
pub fn update_cfg_adapter_with_fn<S: 'static, T: 'static>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn(&S) -> T,
) {
    cfg_adapter.store(lift_to_nullary0(f));
}

/// Updates a StaticCfgAdapter<S, T> with the passed in function, lifting it to a nullary.
/// The lifted passed in function ignores the source applicaton configuration source, which is not invoked.
/// This can be used in testing scenarios and during development where the application configuration
/// source can be [nil_app_cfg].
pub fn update_cfg_adapter_with_const_fn<S, T: 'static>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn() -> T,
) {
    cfg_adapter.store(lift_to_nullary1(f));
}

/// Updates a StaticCfgAdapter<S, T> with a constant closure that always returns the passed in value x.
/// The constant closure ignores the source applicaton configuration source, which is not invoked.
/// This can be used in testing scenarios and during development where the application configuration
/// source can be [nil_app_cfg].
pub fn update_cfg_adapter_with_value<S, T: 'static + Clone + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    x: T,
) {
    cfg_adapter.store(lift_to_nullary2(x));
}

/// Transforms a value into a nullary closure that returns the value.
fn const_closure<T: Clone>(x: T) -> impl Fn() -> T {
    move || x.clone()
}

/// Composes a nullary closure with another closure.
fn compose_nullary<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(S) -> T,
{
    move || g(f())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary(f, g)),
    }
}

/// Composes a nullary closure with another closure.
fn compose_nullary_by_ref<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(&S) -> T,
{
    move || g(&f())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose_by_ref<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(&S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary_by_ref(f, g)),
    }
}

pub fn adapt_by_ref<S, T: Clone, F, G>(f: F, g: G) -> Box<dyn Fn() -> Arc<T>>
where
    F: 'static + Fn() -> Arc<S>,
    G: 'static + Fn(&S) -> T,
{
    let h = move || Arc::new(g(f().deref()));
    Box::new(h)
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns [adapt_by_ref] of the second and third arguments.
pub fn const_or_adapt_by_ref<S, T: 'static + Clone, F, G>(
    k: Option<&T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> Arc<T>>
where
    F: 'static + Fn() -> Arc<S>,
    G: 'static + Fn(&S) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(Arc::new((*k).clone()))),
        None => Box::new(adapt_by_ref(f, g)),
    }
}

/// Composes a nullary closure with another closure.
fn compose_nullary_by_borrow<S, T, F, G>(f: F, g: G) -> impl Fn() -> T
where
    F: Fn() -> S,
    G: Fn(&dyn Borrow<S>) -> T,
{
    move || g(f().borrow())
}

/// Returns the a const closure that returns the first argument if it is not None, otherwise
/// returns the composition of the second and third arguments.
pub fn const_or_compose_by_borrow<S: 'static, T: 'static + Clone, F, G>(
    k: Option<T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> T>
where
    F: 'static + Fn() -> S,
    G: 'static + Fn(&dyn Borrow<S>) -> T,
{
    match k {
        Some(k) => Box::new(const_closure(k)),
        None => Box::new(compose_nullary_by_borrow(f, g)),
    }
}
