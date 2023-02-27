use arc_swap::ArcSwap;
use core::panic;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::type_name;

pub struct CfgDeps<T: 'static, U: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    deps: U,
}

pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

#[derive(Clone)]
struct Cache<T> {
    last_refresh: Instant,
    value: T,
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

    /// Sets a static module CfgDeps with a configuration info source and a dependencies data
    /// structure.
    /// Calls against a mod_cfg_deps after the first call result in a panic.
    pub fn set(
        mod_cfg_deps: &OnceCell<CfgDeps<T, U>>,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        deps: U,
    ) {
        if let Err(_) = mod_cfg_deps.set(CfgDeps::new(cfg_src_fn, deps)) {
            panic!("OnceCell already initialized");
        };
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the deps data structure to the static module CfgDeps.
    /// Calls against a mod_cfg_deps after the first call do not modify the mod_cfg_deps but
    /// log a message.
    pub fn set_with_cfg_adapter<S, F, G>(
        mod_cfg_deps: &OnceCell<CfgDeps<T, U>>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
    ) where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let cache_cell = ArcSwap::new(Arc::new(Cache {
            last_refresh: Instant::now(),
            value: Arc::new(g(f().deref())),
        }));

        let h = move || {
            if let RefreshMode::Refreshable(cache_ttl) = refresh_mode {
                let elapsed = cache_cell.load().last_refresh.elapsed();
                if elapsed > cache_ttl {
                    cache_cell.store(Arc::new(Cache {
                        last_refresh: Instant::now(),
                        value: Arc::new(g(f().deref())),
                    }));
                }
            }

            cache_cell.load().value.clone()
        };

        let deps_str = type_name(&deps);

        match mod_cfg_deps.set(CfgDeps {
            src: Box::new(h),
            deps: deps,
        }) {
            Ok(_) => {
                println!(
                    "OnceCell {:p} initialized with deps {}",
                    mod_cfg_deps, deps_str,
                )
            }
            Err(_) => {
                println!(
                    "Attempt to reinitialize OnceCell {:p} with deps {}",
                    mod_cfg_deps, deps_str,
                );
            }
        }
    }
}
