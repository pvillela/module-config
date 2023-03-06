use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct InnerMut<T, U, I>(RefCell<I>, PhantomData<T>, PhantomData<U>)
where
    T: Clone,
    U: Clone,
    I: CfgDepsMut<T, U> + Clone + core::fmt::Debug;

#[derive(Clone)]
pub struct CfgDepsStd<T, U> {
    src: Arc<dyn 'static + Fn() -> T + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<T>,
    deps: U,
    _t: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

#[derive(Debug, Clone)]
struct Cache<V> {
    last_refresh: Instant,
    value: V,
}

pub trait CfgDeps<T: Clone, U: Clone> {
    /// Returns a pair containing an Arc of the configuration data and the dependencies data structure.
    /// Although the reference to self is immutable, the receiver may have interior mutability and
    /// update a configuration data cache as a result of this call.
    fn get(&self) -> (T, U);

    /// Sets a static module CfgDeps with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> T + Send + Sync,
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
    fn get(&mut self) -> (T, U, bool);

    fn replace(&mut self, other: Self);

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> T + Send + Sync,
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

impl<T, U> core::fmt::Debug for CfgDepsStd<T, U>
where
    T: core::fmt::Debug,
    U: core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = format!(
            "<refresh_mode: {:?}, cache: {:?}, deps: {:?}>",
            self.refresh_mode, self.cache, self.deps,
        );
        f.write_str(&txt)
    }
}

impl<T, U> CfgDepsStd<T, U>
where
    T: Clone,
    U: Clone,
{
    pub fn get(&mut self) -> (T, U, bool) {
        let (cfg, mutated) = self.cfg();
        let deps = self.deps.clone();
        (cfg, deps, mutated)
    }

    pub fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
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
            _t: PhantomData,
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

    fn cfg(&mut self) -> (T, bool) {
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
        let src = move || g(&f());
        Self::new(src, refresh_mode, deps)
    }
}

impl<T, U> CfgDepsMut<T, U> for CfgDepsStd<T, U>
where
    T: Clone,
    U: Clone,
{
    fn get(&mut self) -> (T, U, bool) {
        let (cfg, mutated) = self.cfg();
        let deps = self.deps.clone();
        (cfg, deps, mutated)
    }

    fn replace(&mut self, other: Self) {
        *self = other;
    }

    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> T + Send + Sync,
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

impl<T, U, I> InnerMut<T, U, I>
where
    T: Clone,
    U: Clone,
    I: CfgDepsMut<T, U> + Clone + core::fmt::Debug,
{
    fn get_inner(&self) -> RefCell<I> {
        let inner = self.0.clone();
        // println!(">>> get_inner: {:?}", inner);
        inner
    }

    fn get_inner_clone(&self) -> I {
        let inner = self.get_inner();
        let inner = inner.into_inner();
        inner
    }

    fn set_inner(&self, inner: I) {
        // println!("<<< set_inner: {:?}", inner);
        self.0.replace(inner);
    }

    fn new_priv(inner: I) -> Self {
        InnerMut(RefCell::new(inner.into()), PhantomData, PhantomData)
    }

    pub fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, RefreshMode, U) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> T + Send + Sync,
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

    pub fn get(&self) -> (T, U) {
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
        src: impl 'static + Fn() -> T + Send + Sync,
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

impl<T, U, I> CfgDeps<T, U> for InnerMut<T, U, I>
where
    T: Clone,
    U: Clone,
    I: CfgDepsMut<T, U> + Clone + core::fmt::Debug,
{
    fn get(&self) -> (T, U) {
        Self::get(self)
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        src: impl 'static + Fn() -> T + Send + Sync,
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

pub type CfgDepsInnerMut<T, U> = InnerMut<Rc<T>, U, CfgDepsStd<Rc<T>, U>>;

impl<T, U> CfgDepsInnerMut<T, U>
where
    T: Clone + core::fmt::Debug,
    U: Clone + core::fmt::Debug,
{
    pub fn new(
        src: impl 'static + Fn() -> Rc<T> + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_f(src, refresh_mode, deps, CfgDepsStd::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> Rc<T> + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, CfgDepsStd::new_with_cfg_adapter)
    }
}
