use arc_swap::ArcSwap;
use std::sync::Arc;

/// Wrapped type for a static atomic mutable variable that holds a higher-order function
/// that transforms fn() -> Arc<S> to Fn() -> Arc<T>
pub type StaticCfgAdapter<S, T> = ArcSwap<
    Box<
        dyn 'static
            + Send
            + Sync
            + Fn(fn() -> Arc<S>) -> Box<dyn 'static + Send + Sync + Fn() -> Arc<T>>,
    >,
>;

/// Wrapped type for a higher-order function that transforms fn() -> Arc<S> to Fn() -> Arc<T>
type ArcedCfgAdapter<S, T> = Arc<
    Box<
        dyn 'static
            + Send
            + Sync
            + Fn(fn() -> Arc<S>) -> Box<dyn 'static + Send + Sync + Fn() -> Arc<T>>,
    >,
>;

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
fn lift_to_nullary1<S: 'static, T: 'static>(f: fn() -> T) -> ArcedCfgAdapter<S, T> {
    Arc::new(Box::new(move |_s_src: fn() -> Arc<S>| {
        Box::new(move || Arc::new(f()))
    }))
}

/// Higher-order function that transforms a T to an ArcedCfgAdapter<S, T>
fn lift_to_nullary2<S: 'static, T: 'static + Clone + Send + Sync>(x: T) -> ArcedCfgAdapter<S, T> {
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
pub fn update_cfg_adapter_with_fn<S: 'static, T: 'static + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn(&S) -> T,
) {
    cfg_adapter.store(lift_to_nullary0(f));
}

/// Updates a StaticCfgAdapter<S, T> with the passed in function, lifting it to a nullary.
/// The lifted passed in function ignores the source applicaton configuration source, which is not invoked.
/// This can be used in testing scenarios and during development where the application configuration
/// source can be [nil_app_cfg].
pub fn update_cfg_adapter_with_const_fn<S: 'static, T: 'static + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    f: fn() -> T,
) {
    cfg_adapter.store(lift_to_nullary1(f));
}

/// Updates a StaticCfgAdapter<S, T> with a constant closure that always returns the passed in value x.
/// The constant closure ignores the source applicaton configuration source, which is not invoked.
/// This can be used in testing scenarios and during development where the application configuration
/// source can be [nil_app_cfg].
pub fn update_cfg_adapter_with_value<S: 'static, T: 'static + Clone + Send + Sync>(
    cfg_adapter: &StaticCfgAdapter<S, T>,
    x: T,
) {
    cfg_adapter.store(lift_to_nullary2(x));
}
