use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct CfgDeps<T: 'static, U: 'static + Clone> {
    src: Arc<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: ArcSwap<Option<Cache<T>>>,
    deps: U,
}

#[derive(Clone)]
pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

#[derive(Debug, Clone)]
struct Cache<T> {
    last_refresh: Instant,
    value: Arc<T>,
}

impl<T: 'static + std::fmt::Debug + Clone + Send + Sync, U: 'static + Clone> CfgDeps<T, U> {
    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Arc<Self> {
        CfgDeps {
            src: Arc::new(src),
            refresh_mode,
            cache: ArcSwap::new(None.into()),
            deps,
        }
        .into()
    }

    fn refresh(&self) -> Arc<T> {
        let cfg_value = (self.src)();
        let new_cache = Cache {
            last_refresh: Instant::now(),
            value: cfg_value.clone(),
        };
        self.cache.store(Some(new_cache).into());
        cfg_value
    }

    pub fn cfg(&self) -> Arc<T> {
        let cache_as = &self.cache;
        let cache = &*cache_as.load().clone();
        match &cache {
            Some(cache) => {
                let use_cached = match self.refresh_mode {
                    RefreshMode::NoRefresh => true,
                    RefreshMode::Refreshable(cache_ttl) => {
                        cache.last_refresh.elapsed() <= cache_ttl
                    }
                };
                if use_cached {
                    cache.value.clone()
                } else {
                    self.refresh()
                }
            }
            None => self.refresh(),
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

    pub fn get(mod_cfg_deps: &Lazy<ArcSwap<CfgDeps<T, U>>>) -> (Arc<T>, U) {
        let cfg_deps = mod_cfg_deps.deref().load();
        let cfg = cfg_deps.cfg();
        let deps = cfg_deps.deps.clone();
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
        let cfg_deps = mod_cfg_deps.deref().load();
        let cache_arc = cfg_deps.cache.load().clone();
        mod_cfg_deps.store(
            CfgDeps {
                src: cfg_deps.src.clone(),
                refresh_mode,
                cache: ArcSwap::new(cache_arc),
                deps: cfg_deps.deps.clone(),
            }
            .into(),
        );
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
