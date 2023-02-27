use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct CfgDeps<T: 'static, U: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    cache: ArcSwap<Cache<T>>,
    deps: U,
}

#[derive(Clone)]
pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

#[derive(Clone)]
struct InnerCache<T> {
    last_refresh: Instant,
    value: Arc<T>,
}

#[derive(Clone)]
struct Cache<T> {
    refresh_mode: RefreshMode,
    inner: Option<InnerCache<T>>,
}

impl<T: 'static + Clone + Send + Sync, U: 'static> CfgDeps<T, U> {
    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Arc<Self> {
        CfgDeps {
            src: Box::new(src),
            cache: ArcSwap::new(
                Cache {
                    refresh_mode,
                    inner: None,
                }
                .into(),
            ),
            deps,
        }
        .into()
    }

    pub fn cfg(&self) -> Arc<T> {
        let cache = self.cache.load();
        match cache.refresh_mode {
            RefreshMode::NoRefresh => (self.src)(),
            RefreshMode::Refreshable(cache_ttl) => {
                let refresh = move || -> Arc<T> {
                    let cfg_value = (self.src)();
                    let new_inner = InnerCache {
                        last_refresh: Instant::now(),
                        value: cfg_value,
                    };
                    let new_cache = Cache {
                        refresh_mode: cache.refresh_mode,
                        inner: Some(new_inner),
                    };
                    self.cache.store(new_cache.into());
                    cfg_value
                };

                match cache.inner {
                    Some(inner) => {
                        let elapsed = inner.last_refresh.elapsed();
                        if elapsed > cache_ttl {
                            refresh()
                        } else {
                            inner.value
                        }
                    }
                    None => refresh(),
                }
            }
        }
    }

    pub fn new_with_cfg_adapter<S, F, G>(
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Arc<Self>
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || Arc::new(g(&f()));
        Self::new(src, refresh_mode, deps)
    }

    pub fn get(mod_cfg_deps: &Lazy<ArcSwap<CfgDeps<T, U>>>) -> (Arc<T>, &U) {
        let cfg_deps = mod_cfg_deps.load();
        let cfg = (cfg_deps.src)();
        let deps = &cfg_deps.deps;
        (cfg, deps)
    }

    /// Sets a static module CfgDeps with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    pub fn set(
        mod_cfg_deps: &Lazy<ArcSwap<CfgDeps<T, U>>>,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        mod_cfg_deps.store(CfgDeps::new(cfg_src_fn, refresh_mode, deps));
    }

    pub fn update_refresh_mode(
        mod_cfg_deps: &Lazy<ArcSwap<CfgDeps<T, U>>>,
        refresh_mode: RefreshMode,
    ) {
        let cfg_deps = mod_cfg_deps.load();
        Self::set(mod_cfg_deps, cfg_deps.src, refresh_mode, cfg_deps.deps)
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the static module CfgDeps.
    pub fn set_with_cfg_adapter<S, F, G>(
        mod_cfg_deps: &Lazy<ArcSwap<CfgDeps<T, U>>>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
    ) where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        mod_cfg_deps.store(CfgDeps::new_with_cfg_adapter(f, g, refresh_mode, deps));
    }
}
