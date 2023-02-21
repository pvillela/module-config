use core::panic;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct CfgSrc<T: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
}

impl<T: 'static> CfgSrc<T> {
    fn new(src: impl 'static + Fn() -> Arc<T> + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
    }

    pub fn nil() -> Self {
        Self::new(nil_cfg_src_fn)
    }

    pub fn get(&self) -> Arc<T> {
        self.src.as_ref()()
    }

    pub fn get_from_static(mod_cfg_src: &OnceCell<CfgSrc<T>>) -> Arc<T> {
        mod_cfg_src
            .get()
            .expect("FOO_SFL_CFG_SRC not initialized")
            .get()
    }
}

fn nil_cfg_src_fn<T: 'static>() -> T {
    panic!("Module used before being initialized");
}

pub fn update_cfg_src_with_fn<T: 'static>(
    cfg_src_static: &OnceCell<CfgSrc<T>>,
    cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
) {
    if let Err(_) = cfg_src_static.set(CfgSrc::new(cfg_src_fn)) {
        panic!("OnceCell already initialized");
    };
}

pub enum RefreshMode {
    Cached,
    Refreshable,
}

/// Composes an application info source f with an adapter g for a particular module, then
/// sets the static module config source.
pub fn adapt_by_ref<S, T: Clone + Send + Sync, F, G>(
    f: F,
    g: G,
    refresh_mode: RefreshMode,
    mod_cfg_src: &OnceCell<CfgSrc<T>>,
) where
    F: 'static + Fn() -> Arc<S> + Send + Sync,
    G: 'static + Fn(&S) -> T + Send + Sync,
{
    let cache: Option<Arc<T>> = match refresh_mode {
        RefreshMode::Cached => Some(Arc::new(g(f().deref()))),
        RefreshMode::Refreshable => None,
    };

    let h = move || match cache.clone() {
        Some(v) => v,
        None => Arc::new(g(f().deref())),
    };

    if let Err(_) = mod_cfg_src.set(CfgSrc { src: Box::new(h) }) {
        panic!("OnceCell already initialized");
    };
}
