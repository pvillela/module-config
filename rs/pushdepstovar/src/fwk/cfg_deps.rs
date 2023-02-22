use core::panic;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct CfgDeps<T: 'static, U: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    deps: U,
}

pub enum RefreshMode {
    Cached,
    Refreshable,
}

impl<T: 'static + Clone + Send + Sync, U: 'static> CfgDeps<T, U> {
    fn new(src: impl 'static + Fn() -> Arc<T> + Send + Sync, deps: U) -> Self {
        CfgDeps {
            src: Box::new(src),
            deps,
        }
    }

    pub fn cfg(&self) -> Arc<T> {
        self.src.as_ref()()
    }

    pub fn get(mod_cfg_src: &OnceCell<CfgDeps<T, U>>) -> (Arc<T>, &U) {
        let cfg_deps = mod_cfg_src
            .get()
            .expect("module config source static not initialized");
        let cfg = (cfg_deps.src)();
        let deps = &cfg_deps.deps;
        (cfg, deps)
    }

    pub fn set(
        mod_cfg_src: &OnceCell<CfgDeps<T, U>>,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        deps: U,
    ) {
        if let Err(_) = mod_cfg_src.set(CfgDeps::new(cfg_src_fn, deps)) {
            panic!("OnceCell already initialized");
        };
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets the static module CfgDeps.
    pub fn set_with_cfg_adapter<S, F, G>(
        mod_cfg_src: &OnceCell<CfgDeps<T, U>>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
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

        if let Err(_) = mod_cfg_src.set(CfgDeps {
            src: Box::new(h),
            deps,
        }) {
            panic!("OnceCell already initialized");
        };
    }
}
