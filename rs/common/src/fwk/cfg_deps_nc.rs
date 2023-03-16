use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use super::RefreshMode;

pub trait InnerMutNc<I> {
    fn get_inner_clone(&self) -> I;

    fn set_inner(&self, inner: I);

    fn from(_: I) -> Self;

    fn with<V>(&self, _: impl Fn(&I) -> V) -> V;
}

impl<I: Clone> InnerMutNc<I> for ArcSwap<I> {
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

impl<I: Clone> InnerMutNc<I> for RefCell<I> {
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

pub struct CfgDepsInnerMutNc<T, TX, U, I, IM>(
    IM,
    U,
    PhantomData<T>,
    PhantomData<TX>,
    PhantomData<I>,
)
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgDepsMutNc<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMutNc<I>;

#[derive(Clone)]
pub struct CfgRaw<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    src: Arc<dyn 'static + Fn() -> T + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<TX>,
}

#[derive(Debug, Clone)]
struct Cache<V> {
    last_refresh: Instant,
    value: V,
}

pub trait CfgDepsImmutNc<T, TX: Clone, U> {
    fn get_cfg(&self) -> TX;

    fn get_deps(&self) -> &U;

    /// Returns a pair containing an Arc of the configuration data and the dependencies data structure.
    /// Although the reference to self is immutable, the receiver may have interior mutability and
    /// update a configuration data cache as a result of this call.
    fn get_cfg_deps(&self) -> (TX, &U) {
        (self.get_cfg(), self.get_deps())
    }

    /// Sets a static module CfgDepsNc with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    );

    fn update_refresh_mode(&self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the static module CfgDepsNc.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync;
}

pub trait CfgDepsMutNc<T, TX: Clone> {
    /// Returns the configuration data in the cache, even if stale.
    fn get_cfg_cached(&self) -> TX;

    fn cache_expired(&self) -> bool;

    /// This will return the current configuration data, according to the object's cache refresh policy,
    /// with a possible change to cache state as a side-effect.
    fn get_cfg(&mut self) -> TX;

    fn replace(&mut self, other: Self);

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    );

    fn update_refresh_mode(&mut self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&mut self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync;
}

impl<T, TX> core::fmt::Debug for CfgRaw<T, TX>
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

impl<T, TX> CfgRaw<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    fn new(src: impl 'static + Fn() -> T + Send + Sync, refresh_mode: RefreshMode) -> Self {
        let cfg = src();
        CfgRaw {
            src: Arc::new(src),
            refresh_mode,
            cache: Cache {
                last_refresh: Instant::now(),
                value: cfg.into(),
            },
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

    fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || g(&f());
        Self::new(src, refresh_mode)
    }
}

impl<T, TX> CfgDepsMutNc<T, TX> for CfgRaw<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    fn get_cfg_cached(&self) -> TX {
        self.cache.value.clone()
    }

    fn cache_expired(&self) -> bool {
        // println!("refresh_mode={:?}", self.refresh_mode);
        let res = match self.refresh_mode {
            RefreshMode::NoRefresh => false,
            RefreshMode::Refreshable(cache_ttl) => {
                // println!(
                //     "cache.last_refresh.elapsed()={:?}, cache_ttl={:?}",
                //     self.cache.last_refresh.elapsed(),
                //     cache_ttl
                // );
                if self.cache.last_refresh.elapsed() > cache_ttl {
                    true
                } else {
                    false
                }
            }
        };
        // println!("cache_expired={}", res);
        res
    }

    fn get_cfg(&mut self) -> TX {
        if self.cache_expired() {
            self.refresh();
        }
        self.cache.value.clone()
    }

    fn replace(&mut self, other: Self) {
        *self = other;
    }

    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    ) {
        self.replace(Self::new(src, refresh_mode));
    }

    fn update_refresh_mode(&mut self, refresh_mode: RefreshMode) {
        self.refresh_mode = refresh_mode;
    }

    fn update_with_cfg_adapter<S, F, G>(&mut self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        self.replace(Self::new_with_cfg_adapter(f, g, refresh_mode));
    }
}

impl<T, TX, U, I, IM> CfgDepsInnerMutNc<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgDepsMutNc<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMutNc<I>,
{
    // I don't understand why I have to do this as this method is defined in trait CfgDepsImmutNc.
    pub fn get_cfg(&self) -> TX {
        CfgDepsImmutNc::get_cfg(self)
    }

    // I don't understand why I have to do this as this method is defined in trait CfgDepsImmutNc.
    pub fn get_deps<'a>(&'a self) -> &'a U {
        &CfgDepsImmutNc::get_deps(self)
    }

    // I don't understand why I have to do this as this method is defined in trait CfgDepsImmutNc.
    pub fn get_cfg_deps<'a>(&'a self) -> (TX, &'a U) {
        CfgDepsImmutNc::get_cfg_deps(self)
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

    fn new_priv(inner: I, deps: U) -> Self {
        CfgDepsInnerMutNc(IM::from(inner), deps, PhantomData, PhantomData, PhantomData)
    }

    pub fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, RefreshMode) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> T + Send + Sync,
    {
        Self::new_priv(factory(src, refresh_mode), deps)
    }

    pub fn new_with_cfg_adapter_f<S, F, G>(
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, G, RefreshMode) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_priv(factory(f, g, refresh_mode), deps)
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    pub fn update_all(
        &self,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    ) {
        let mut inner = self.get_inner_clone();
        inner.update_all(src, refresh_mode);
        self.set_inner(inner);
    }

    pub fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        let mut inner = self.get_inner_clone();
        inner.update_refresh_mode(refresh_mode);
        self.set_inner(inner);
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    pub fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let mut inner = self.get_inner_clone();
        inner.update_with_cfg_adapter(f, g, refresh_mode);
        self.set_inner(inner);
    }
}

// impl<T, TX, U, I, IM> Clone for CfgDepsInnerMutNc<T, TX, U, I, IM>
// where
//     TX: From<T> + Clone + core::fmt::Debug,
//     I: CfgDepsMutNc<T, TX> + Clone + core::fmt::Debug,
//     IM: InnerMutNc<I>,
// {
//     fn clone(&self) -> Self {
//         let inner = self.get_inner_clone();
//         Self::new_priv(inner)
//     }
// }

impl<T, TX, U, I, IM> CfgDepsImmutNc<T, TX, U> for CfgDepsInnerMutNc<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgDepsMutNc<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMutNc<I>,
{
    fn get_cfg(&self) -> TX {
        let inner = self.get_inner();

        let f_cache_expired = move |i: &I| -> bool { i.cache_expired() };

        let f_cfg_cached = move |i: &I| -> TX { i.get_cfg_cached() };

        let cache_expired = inner.with(f_cache_expired);

        if cache_expired {
            let mut inner = self.get_inner_clone();
            let cfg = inner.get_cfg();
            self.set_inner(inner);
            cfg
        } else {
            inner.with(f_cfg_cached)
        }
    }

    fn get_deps(&self) -> &U {
        &self.1
    }

    /// Updates the receiver with a configuration info source, refresh mode, and a dependencies data
    /// structure.
    fn update_all(&self, src: impl 'static + Fn() -> T + Send + Sync, refresh_mode: RefreshMode) {
        Self::update_all(self, src, refresh_mode)
    }

    fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        Self::update_refresh_mode(&self, refresh_mode)
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode and deps data structure to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::update_with_cfg_adapter(&self, f, g, refresh_mode)
    }
}

// Type aliases for CfgDepsNc.

pub type CfgDepsNc<T, TX, U, IM> = CfgDepsInnerMutNc<T, TX, U, CfgRaw<T, TX>, IM>;

pub type CfgDepsRefCellNc<T, TX, U> =
    CfgDepsInnerMutNc<T, TX, U, CfgRaw<T, TX>, RefCell<CfgRaw<T, TX>>>;

pub type CfgDepsArcSwapNc<T, TX, U> =
    CfgDepsInnerMutNc<T, TX, U, CfgRaw<T, TX>, ArcSwap<CfgRaw<T, TX>>>;

pub type CfgDepsRefCellRcNc<T, U> = CfgDepsRefCellNc<T, Rc<T>, U>;

pub type CfgDepsArcSwapRcNc<T, U> = CfgDepsArcSwapNc<T, Rc<T>, U>;

pub type CfgDepsRefCellArcNc<T, U> = CfgDepsRefCellNc<T, Arc<T>, U>;

pub type CfgDepsArcSwapArcNc<T, U> = CfgDepsArcSwapNc<T, Arc<T>, U>;

pub type CfgDepsRefCellIdNc<T, U> = CfgDepsRefCellNc<T, T, U>;

pub type CfgDepsArcSwapIdNc<T, U> = CfgDepsArcSwapNc<T, T, U>;

pub type CfgDepsArcNc<T, U> = CfgDepsArcSwapArcNc<T, U>;

// pub type CfgDepsDefaultNc<T, U> = CfgDepsArcSwapArcNc<T, U>;
// pub type CfgDepsDefaultNc<T, U> = CfgDepsRefCellArcNc<T, U>;
pub type CfgDepsDefaultNc<T, U> = CfgDepsRefCellRcNc<T, U>;

impl<T, TX, U, IM> CfgDepsNc<T, TX, U, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMutNc<CfgRaw<T, TX>>,
{
    pub fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_f(src, refresh_mode, deps, CfgRaw::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, CfgRaw::new_with_cfg_adapter)
    }
}

impl<T, TX, U, IM> CfgDepsNc<T, TX, U, IM>
where
    T: 'static + Clone + Send + Sync,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMutNc<CfgRaw<T, TX>>,
{
    pub fn new_with_const_or_cfg_adapter<S, F, G>(
        k: Option<&'static T>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        match k {
            Some(k) => {
                let src = move || k.clone();
                Self::new(src, refresh_mode, deps)
            }
            None => Self::new_with_cfg_adapter(f, g, refresh_mode, deps),
        }
    }
}
