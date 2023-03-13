use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub trait InnerMut<I> {
    fn get_inner_clone(&self) -> I;

    fn set_inner(&self, inner: I);

    fn from(_: I) -> Self;

    fn with<V>(&self, _: impl Fn(&I) -> V) -> V;
}

impl<I: Clone> InnerMut<I> for ArcSwap<I> {
    fn get_inner_clone(&self) -> I {
        let inner = self.load().as_ref().clone();
        inner
    }

    fn set_inner(&self, inner: I) {
        // println!("<<< set_inner: {:?}", inner);
        self.store(Arc::new(inner));
    }

    fn from(x: I) -> Self {
        ArcSwap::new(Arc::new(x))
    }

    fn with<V>(&self, f: impl Fn(&I) -> V) -> V {
        let x = self.load();
        f(&x)
    }
}

impl<I: Clone> InnerMut<I> for RefCell<I> {
    fn get_inner_clone(&self) -> I {
        let inner = self.borrow().clone();
        inner
    }

    fn set_inner(&self, inner: I) {
        // println!("<<< set_inner: {:?}", inner);
        self.replace(inner);
    }

    fn from(x: I) -> Self {
        RefCell::new(x)
    }

    fn with<V>(&self, f: impl Fn(&I) -> V) -> V {
        let x = self.borrow();
        f(&x)
    }
}

pub struct CfgDepsInnerMut<T, TX, U, I, IM>(
    IM,
    PhantomData<T>,
    PhantomData<TX>,
    PhantomData<U>,
    PhantomData<I>,
)
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    I: CfgDepsMut<T, TX, U> + Clone + core::fmt::Debug,
    IM: InnerMut<I>;

#[derive(Clone)]
pub struct CfgDepsRaw<T, TX, U>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    src: Arc<dyn 'static + Fn() -> T + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<TX>,
    deps: U,
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

pub trait CfgDepsImmut<T, TX: Clone, U: Clone> {
    fn get_cfg(&self) -> TX;

    fn get_deps(&self) -> U;

    /// Returns a pair containing an Arc of the configuration data and the dependencies data structure.
    /// Although the reference to self is immutable, the receiver may have interior mutability and
    /// update a configuration data cache as a result of this call.
    fn get(&self) -> (TX, U) {
        (self.get_cfg(), self.get_deps())
    }

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

pub trait CfgDepsMut<T, TX: Clone, U: Clone> {
    /// Returns the configuration data in the cache, even if stale.
    fn get_cfg_cached(&self) -> TX;

    fn cache_expired(&self) -> bool;

    /// This will return the current configuration data, according to the object's cache refresh policy,
    /// with a possible change to cache state as a side-effect.
    fn get_cfg(&mut self) -> TX;

    fn get_deps(&self) -> U;

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

impl<T, TX, U> core::fmt::Debug for CfgDepsRaw<T, TX, U>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = format!(
            "<refresh_mode: {:?}, cache: {:?}>",
            self.refresh_mode, self.cache,
        );
        f.write_str(&txt)
    }
}

impl<T, TX, U> CfgDepsRaw<T, TX, U>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
{
    fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        let cfg = src();
        CfgDepsRaw {
            src: Arc::new(src),
            refresh_mode,
            cache: Cache {
                last_refresh: Instant::now(),
                value: cfg.into(),
            },
            deps,
        }
    }

    /// Function to update self with a refreshed the cache.
    fn refresh(&mut self) {
        let cfg_value: TX = (self.src)().into();
        let cache = Cache {
            last_refresh: Instant::now(),
            value: cfg_value.clone(),
        };
        self.cache = cache;
    }

    fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || g(&f());
        Self::new(src, refresh_mode, deps)
    }
}

impl<T, TX, U> CfgDepsMut<T, TX, U> for CfgDepsRaw<T, TX, U>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
{
    fn get_cfg_cached(&self) -> TX {
        self.cache.value.clone()
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

    fn get_cfg(&mut self) -> TX {
        if self.cache_expired() {
            self.refresh();
        }
        self.cache.value.clone()
    }

    fn get_deps(&self) -> U {
        self.deps.clone()
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

impl<T, TX, U, I, IM> CfgDepsInnerMut<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    I: CfgDepsMut<T, TX, U> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    // I don't understand why I have to do this as this method is defined in trait CfgDepsImmut.
    pub fn get(&self) -> (TX, U) {
        CfgDepsImmut::get(self)
    }

    fn get_inner(&self) -> &IM {
        &self.0
    }

    fn get_inner_clone(&self) -> I {
        self.0.get_inner_clone()
    }

    fn set_inner(&self, inner: I) {
        self.0.set_inner(inner);
    }

    fn new_priv(inner: I) -> Self {
        CfgDepsInnerMut(
            IM::from(inner),
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
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

impl<T, TX, U, I, IM> Clone for CfgDepsInnerMut<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    I: CfgDepsMut<T, TX, U> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    fn clone(&self) -> Self {
        let inner = self.get_inner_clone();
        Self::new_priv(inner)
    }
}

impl<T, TX, U, I, IM> CfgDepsImmut<T, TX, U> for CfgDepsInnerMut<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    I: CfgDepsMut<T, TX, U> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    fn get_cfg(&self) -> TX {
        let inner = self.get_inner();
        let f = move |inner: &I| -> TX {
            if inner.cache_expired() {
                let mut inner = inner.clone();
                let cfg = inner.get_cfg();
                self.set_inner(inner);
                cfg
            } else {
                inner.get_cfg_cached()
            }
        };
        inner.with(f)
    }

    fn get_deps(&self) -> U {
        let inner = self.get_inner();
        let f = move |inner: &I| -> U { inner.get_deps() };
        inner.with(f)
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

// Type aliases for CfgDeps.

pub type CfgDeps<T, TX, U, IM> = CfgDepsInnerMut<T, TX, U, CfgDepsRaw<T, TX, U>, IM>;

pub type CfgDepsRefCell<T, TX, U> =
    CfgDepsInnerMut<T, TX, U, CfgDepsRaw<T, TX, U>, RefCell<CfgDepsRaw<T, TX, U>>>;

pub type CfgDepsArcSwap<T, TX, U> =
    CfgDepsInnerMut<T, TX, U, CfgDepsRaw<T, TX, U>, ArcSwap<CfgDepsRaw<T, TX, U>>>;

pub type CfgDepsRefCellRc<T, U> = CfgDepsRefCell<T, Rc<T>, U>;

pub type CfgDepsArcSwapRc<T, U> = CfgDepsArcSwap<T, Rc<T>, U>;

pub type CfgDepsRefCellArc<T, U> = CfgDepsRefCell<T, Arc<T>, U>;

pub type CfgDepsArcSwapArc<T, U> = CfgDepsArcSwap<T, Arc<T>, U>;

pub type CfgDepsRefCellId<T, U> = CfgDepsRefCell<T, T, U>;

pub type CfgDepsArcSwapId<T, U> = CfgDepsArcSwap<T, T, U>;

pub type CfgDepsArc<T, U> = CfgDepsArcSwapArc<T, U>;

// pub type CfgDepsDefault<T, U> = CfgDepsArcSwapArc<T, U>;
// pub type CfgDepsDefault<T, U> = CfgDepsRefCellArc<T, U>;
pub type CfgDepsDefault<T, U> = CfgDepsRefCellRc<T, U>;

impl<T, TX, U, IM> CfgDeps<T, TX, U, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    IM: InnerMut<CfgDepsRaw<T, TX, U>>,
{
    pub fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_f(src, refresh_mode, deps, CfgDepsRaw::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, CfgDepsRaw::new_with_cfg_adapter)
    }
}
