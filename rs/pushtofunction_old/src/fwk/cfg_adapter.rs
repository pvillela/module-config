use std::sync::Arc;

/// Transforms a value into a nullary closure that returns the value.
pub fn const_closure<T: Clone + Send + Sync>(x: T) -> impl Fn() -> T + Send + Sync {
    move || x.clone()
}

/// Function that can be used as a placeholder for a configuration source during development.
/// Supports any configuration info type and panics if called.
pub fn nil_app_cfg<T>() -> T {
    todo!("Configuration source not provided.")
}

/// Composes an application info source f with an adapter g for a particular module.
pub fn adapt_by_ref<S, T: Clone, F, G>(f: F, g: G) -> Box<dyn Fn() -> Arc<T> + Send + Sync>
where
    F: 'static + Fn() -> S + Send + Sync,
    G: 'static + Fn(&S) -> T + Send + Sync,
{
    let h = move || Arc::new(g(&f()));
    Box::new(h)
}

/// Returns the a const closure that returns the Arc of the deref of the first argument if it is not None,
/// otherwise returns [adapt_by_ref] of the second and third arguments.
pub fn const_or_adapt_by_ref<S, T: 'static + Clone + Send + Sync, F, G>(
    k: Option<&T>,
    f: F,
    g: G,
) -> Box<dyn Fn() -> Arc<T> + Send + Sync>
where
    F: 'static + Fn() -> S + Send + Sync,
    G: 'static + Fn(&S) -> T + Send + Sync,
{
    match k {
        Some(k) => Box::new(const_closure(Arc::new((*k).clone()))),
        None => Box::new(adapt_by_ref(f, g)),
    }
}
