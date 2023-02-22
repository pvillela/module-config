use core::panic;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct CfgDepsSrc<T: 'static, U: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    deps: U,
}

impl<T: 'static, U: 'static> CfgDepsSrc<T, U> {
    fn new(src: impl 'static + Fn() -> Arc<T> + Send + Sync, deps: U) -> Self {
        CfgDepsSrc {
            src: Box::new(src),
            deps,
        }
    }

    pub fn get(&self) -> Arc<T> {
        self.src.as_ref()()
    }

    pub fn get_from_static(mod_cfg_src: &OnceCell<CfgDepsSrc<T, U>>) -> (Arc<T>, &U) {
        let cfg_deps = mod_cfg_src
            .get()
            .expect("module config source static not initialized");
        let cfg = (cfg_deps.src)();
        let deps = &cfg_deps.deps;
        (cfg, deps)
    }
}

pub fn update_cfg_src_with_fn<T: 'static, U: 'static>(
    cfg_src_static: &OnceCell<CfgDepsSrc<T, U>>,
    cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
    deps: U,
) {
    if let Err(_) = cfg_src_static.set(CfgDepsSrc::new(cfg_src_fn, deps)) {
        panic!("OnceCell already initialized");
    };
}

pub enum RefreshMode {
    Cached,
    Refreshable,
}

/// Composes an application info source f with an adapter g for a particular module, then
/// sets the static module config source.
pub fn adapt_by_ref<S, T: Clone + Send + Sync, U, F, G>(
    f: F,
    g: G,
    refresh_mode: RefreshMode,
    deps: U,
    mod_cfg_src: &OnceCell<CfgDepsSrc<T, U>>,
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

    if let Err(_) = mod_cfg_src.set(CfgDepsSrc {
        src: Box::new(h),
        deps,
    }) {
        panic!("OnceCell already initialized");
    };
}
