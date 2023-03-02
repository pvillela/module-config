use arc_swap::{ArcSwap, Guard};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct CfgDepsInnerMut<T, U>(ArcSwap<CfgDepsStd<T, U>>);

#[derive(Clone)]
struct CfgDepsStd<T, U> {
    src: Arc<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<T>,
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
    /// Returns a pair containing an Arc of the configuration data and the dependencies data structure.
    /// Although the reference to self is immutable, the receiver may have interior mutability and
    /// update a configuration data cache as a result of this call.
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

pub trait CfgDepsMut<T: Clone, U: Clone> {
    /// Returns a triple containing an Arc of the configuration data, the dependencies data structure,
    /// and an indicator of whether it is true that the object was mutated.
    /// This will return the current configuration data, according to the object's cache refresh policy,
    /// with a possible change to cache state as a side-effect.
    fn get(&mut self) -> (Arc<T>, U, bool);
}

impl<T: Clone, U: Clone> CfgDepsStd<T, U> {
    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        let cfg = src();
        CfgDepsStd {
            src: Arc::new(src),
            refresh_mode,
            cache: Cache {
                last_refresh: Instant::now(),
                value: cfg,
            },
            deps,
        }
    }

    /// Function to produce a copy of self with a refreshed the cache.
    pub fn refresh(&mut self) {
        let cfg_value = (self.src)();
        let cache = Cache {
            last_refresh: Instant::now(),
            value: cfg_value.clone(),
        };
        self.cache = cache;
    }

    fn cache_expired(&self) -> bool {
        if let RefreshMode::Refreshable(cache_ttl) = self.refresh_mode {
            if self.cache.last_refresh.elapsed() > cache_ttl {
                return true;
            }
        }
        false
    }

    fn cfg(&mut self) -> (Arc<T>, bool) {
        if !self.cache_expired() {
            return (self.cache.value.clone(), false);
        }
        self.refresh();
        (self.cache.value.clone(), true)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || Arc::new(g(&f()));
        Self::new(src, refresh_mode, deps)
    }
}

impl<T: Clone, U: Clone> CfgDepsMut<T, U> for CfgDepsStd<T, U> {
    fn get(&mut self) -> (Arc<T>, U, bool) {
        let (cfg, mutated) = self.cfg();
        let deps = self.deps.clone();
        (cfg, deps, mutated)
    }
}

impl<T: Clone, U: Clone> CfgDepsInnerMut<T, U> {
    fn get_inner(&self) -> Guard<Arc<CfgDepsStd<T, U>>> {
        self.0.load()
    }

    fn set_inner(&self, inner: CfgDepsStd<T, U>) {
        self.0.store(Arc::new(inner));
    }

    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        CfgDepsInnerMut(CfgDepsStd::new(src, refresh_mode, deps).into())
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        CfgDepsInnerMut(CfgDepsStd::new_with_cfg_adapter(f, g, refresh_mode, deps).into())
    }
}

impl<T: Clone, U: Clone> CfgDeps<T, U> for CfgDepsInnerMut<T, U> {
    fn get(&self) -> (Arc<T>, U) {
        let inner = &*self.get_inner().clone();
        let mut inner = inner.clone();
        let (cfg, deps, mutated) = inner.get();
        if mutated {
            self.set_inner(inner.clone());
        }
        (cfg, deps)
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        let inner = CfgDepsStd::new(cfg_src_fn, refresh_mode, deps);
        self.set_inner(inner);
    }

    fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        let new_inner = self.get_inner();
        let new_inner = CfgDepsStd {
            src: new_inner.src.clone(),
            refresh_mode,
            cache: new_inner.cache.clone(),
            deps: new_inner.deps.clone(),
        };
        self.set_inner(new_inner);
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let inner = CfgDepsStd::new_with_cfg_adapter(f, g, refresh_mode, deps);
        self.set_inner(inner);
    }
}
