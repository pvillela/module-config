use arc_swap::{ArcSwap, Guard};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct CfgDepsInnerMut<T, U>(ArcSwap<CfgDepsStd<T, U>>);

#[derive(Clone)]
struct CfgDepsStd<T, U> {
    src: Arc<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Option<Cache<T>>,
    deps: U,
}

impl<T, U> Into<ArcSwap<CfgDepsStd<T, U>>> for CfgDepsStd<T, U> {
    fn into(self) -> ArcSwap<CfgDepsStd<T, U>> {
        ArcSwap::new(Arc::new(self))
    }
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

pub trait CfgDeps<T: Clone, U: Clone> {
    fn get(&self) -> (Arc<T>, U);

    /// Sets a static module CfgDeps with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    );

    fn update_refresh_mode(&self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the static module CfgDeps.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync;
}

impl<T: Clone, U: Clone> CfgDepsStd<T, U> {
    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        CfgDepsStd {
            src: Arc::new(src),
            refresh_mode,
            cache: None,
            deps,
        }
    }

    /// Function to produce a copy of self with a refreshed the cache.
    pub fn refreshed(&self) -> (Arc<T>, Self) {
        // let curr_inner = self.0.load();
        let cfg_value = (self.src)();
        let cache = Cache {
            last_refresh: Instant::now(),
            value: cfg_value.clone(),
        };
        (
            cfg_value,
            CfgDepsStd {
                src: self.src.clone(),
                refresh_mode: self.refresh_mode.clone(),
                cache: Some(cache),
                deps: self.deps.clone(),
            },
        )
    }

    fn cfg(&self) -> (Arc<T>, Option<Self>) {
        let cache = self.cache.clone();
        match cache {
            Some(cache) => {
                let use_cached = match &self.refresh_mode {
                    RefreshMode::NoRefresh => true,
                    RefreshMode::Refreshable(cache_ttl) => {
                        cache.last_refresh.elapsed() <= *cache_ttl
                    }
                };
                if use_cached {
                    (cache.value.clone(), None)
                } else {
                    let (cfg_value, new_state) = self.refreshed();
                    (cfg_value, Some(new_state))
                }
            }
            None => {
                let (cfg_value, new_state) = self.refreshed();
                (cfg_value, Some(new_state))
            }
        }
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || Arc::new(g(&f()));
        Self::new(src, refresh_mode, deps)
    }

    pub fn get(&self) -> (Arc<T>, U, Option<CfgDepsStd<T, U>>) {
        let (cfg, new_state) = self.cfg();
        let deps = self.deps.clone();
        (cfg, deps, new_state)
    }
}

impl<T: Clone, U: Clone> CfgDepsInnerMut<T, U> {
    fn set_inner_items(
        &self,
        src: Arc<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
        refresh_mode: RefreshMode,
        cache: Option<Cache<T>>,
        deps: U,
    ) {
        self.0.store(
            CfgDepsStd {
                src,
                refresh_mode,
                cache,
                deps,
            }
            .into(),
        );
    }

    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        CfgDepsInnerMut(
            CfgDepsStd {
                src: Arc::new(src),
                refresh_mode,
                cache: None,
                deps,
            }
            .into(),
        )
    }

    fn get_inner(&self) -> Guard<Arc<CfgDepsStd<T, U>>> {
        self.0.load()
    }

    /// Helper function to refresh the cache and return the cached value. I takes the parameter
    /// curr_inner which can be obtained from self but is passed in to avoid having to call
    /// load again on self.0.
    fn refresh(&self, inner: &Guard<Arc<CfgDepsStd<T, U>>>) -> Arc<T> {
        // let curr_inner = self.0.load();
        let cfg_value = (inner.src)();
        let new_cache = Cache {
            last_refresh: Instant::now(),
            value: cfg_value.clone(),
        };
        let new_inner = CfgDepsStd {
            src: inner.src.clone(),
            refresh_mode: inner.refresh_mode.clone(),
            cache: Some(new_cache),
            deps: inner.deps.clone(),
        };
        self.0.store(new_inner.into());
        cfg_value
    }

    fn cfg(&self, inner: &Guard<Arc<CfgDepsStd<T, U>>>) -> Arc<T> {
        let cache = &inner.cache;
        match cache {
            Some(cache) => {
                let use_cached = match inner.refresh_mode {
                    RefreshMode::NoRefresh => true,
                    RefreshMode::Refreshable(cache_ttl) => {
                        cache.last_refresh.elapsed() <= cache_ttl
                    }
                };
                if use_cached {
                    cache.value.clone()
                } else {
                    self.refresh(inner)
                }
            }
            None => self.refresh(inner),
        }
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || Arc::new(g(&f()));
        Self::new(src, refresh_mode, deps)
    }

    pub fn get(&self) -> (Arc<T>, U) {
        let inner = self.get_inner();
        let cfg = self.cfg(&inner);
        let deps = inner.deps.clone();
        (cfg, deps)
    }

    /// Sets a static module CfgDeps with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    pub fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        self.set_inner_items(Arc::new(cfg_src_fn), refresh_mode, None, deps);
    }

    pub fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        let inner = self.get_inner();
        self.set_inner_items(
            inner.src.clone(),
            refresh_mode,
            inner.cache.clone(),
            inner.deps.clone(),
        );
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the static module CfgDeps.
    pub fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let inner = self.get_inner();
        let src = move || Arc::new(g(&f()));
        self.set_inner_items(Arc::new(src), refresh_mode, inner.cache.clone(), deps);
    }
}
