use arc_swap::{ArcSwap, Guard};
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct InnerMut<T: Clone, U: Clone, I: CfgDepsMut<T, U> + Clone + core::fmt::Debug>(
    ArcSwap<I>,
    PhantomData<T>,
    PhantomData<U>,
);

#[derive(Clone)]
pub struct CfgDepsStd<T, U> {
    src: Arc<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<T>,
    deps: U,
}

impl<T: core::fmt::Debug, U: core::fmt::Debug> core::fmt::Debug for CfgDepsStd<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = format!(
            "<refresh_mode: {:?}, cache: {:?}, deps: {:?}>",
            self.refresh_mode, self.cache, self.deps,
        );
        f.write_str(&txt)
    }
}

impl<T, U> Into<ArcSwap<CfgDepsStd<T, U>>> for CfgDepsStd<T, U> {
    fn into(self) -> ArcSwap<CfgDepsStd<T, U>> {
        ArcSwap::new(Arc::new(self))
    }
}

#[derive(Clone, Debug)]
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

    fn replace(&mut self, other: Self);

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    );

    fn update_refresh_mode(&mut self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&mut self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync;
}

impl<T: Clone, U: Clone> CfgDepsStd<T, U> {
    pub fn get(&mut self) -> (Arc<T>, U, bool) {
        let (cfg, mutated) = self.cfg();
        let deps = self.deps.clone();
        (cfg, deps, mutated)
    }

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
        match self.refresh_mode {
            RefreshMode::NoRefresh => false,
            RefreshMode::Refreshable(cache_ttl) => {
                if self.cache.last_refresh.elapsed() > cache_ttl {
                    true
                } else {
                    false
                }
            }
        }
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

    fn replace(&mut self, other: Self) {
        *self = other;
    }

    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        self.replace(Self::new(src, refresh_mode, deps));
    }

    fn update_refresh_mode(&mut self, refresh_mode: RefreshMode) {
        self.refresh_mode = refresh_mode;
    }

    fn update_with_cfg_adapter<S, F, G>(&mut self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        self.replace(Self::new_with_cfg_adapter(f, g, refresh_mode, deps));
    }
}

impl<T: Clone, U: Clone, I: CfgDepsMut<T, U> + Clone + core::fmt::Debug> InnerMut<T, U, I> {
    fn get_inner(&self) -> Guard<Arc<I>> {
        let inner = self.0.load();
        // println!(">>> get_inner: {:?}", inner);
        inner
    }

    fn get_inner_clone(&self) -> I {
        let inner = &*self.get_inner().clone();
        let inner = inner.clone();
        inner
    }

    fn set_inner(&self, inner: I) {
        // println!("<<< set_inner: {:?}", inner);
        self.0.store(Arc::new(inner));
    }

    fn new_priv(inner: I) -> Self {
        InnerMut(ArcSwap::new(inner.into()), PhantomData, PhantomData)
    }

    pub fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, RefreshMode, U) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<T> + Send + Sync,
    {
        Self::new_priv(factory(src, refresh_mode, deps))
    }

    pub fn new_with_cfg_adapter_f<S, F, G>(
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, G, RefreshMode, U) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_priv(factory(f, g, refresh_mode, deps))
    }

    pub fn get(&self) -> (Arc<T>, U) {
        let mut inner = self.get_inner_clone();
        let (cfg, deps, mutated) = inner.get();
        if mutated {
            self.set_inner(inner.clone());
        }
        (cfg, deps)
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    pub fn update_all(
        &self,
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        let mut inner = self.get_inner_clone();
        inner.update_all(src, refresh_mode, deps);
        self.set_inner(inner);
    }

    pub fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        let mut inner = self.get_inner_clone();
        inner.update_refresh_mode(refresh_mode);
        self.set_inner(inner);
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    pub fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let mut inner = self.get_inner_clone();
        inner.update_with_cfg_adapter(f, g, refresh_mode, deps);
        self.set_inner(inner);
    }
}

impl<T: Clone, U: Clone, I: CfgDepsMut<T, U> + Clone + core::fmt::Debug> CfgDeps<T, U>
    for InnerMut<T, U, I>
{
    fn get(&self) -> (Arc<T>, U) {
        Self::get(self)
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        Self::update_all(self, src, refresh_mode, deps)
    }

    fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        Self::update_refresh_mode(&self, refresh_mode)
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode, deps: U)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::update_with_cfg_adapter(&self, f, g, refresh_mode, deps)
    }
}

pub type CfgDepsInnerMut<S, T> = InnerMut<S, T, CfgDepsStd<S, T>>;

impl<T: Clone + core::fmt::Debug, U: Clone + core::fmt::Debug> CfgDepsInnerMut<T, U> {
    pub fn new(
        src: impl 'static + Fn() -> Arc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_f(src, refresh_mode, deps, CfgDepsStd::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, CfgDepsStd::new_with_cfg_adapter)
    }
}
