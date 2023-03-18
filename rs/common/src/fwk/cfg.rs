use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

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

pub struct CfgInnerMut<T, TX, I, IM>(IM, PhantomData<T>, PhantomData<TX>, PhantomData<I>)
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgMut<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMut<I>;

#[derive(Clone)]
pub struct CfgRaw<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    src: Arc<dyn 'static + Fn() -> T + Send + Sync>,
    refresh_mode: RefreshMode,
    cache: Cache<TX>,
}

pub trait CfgImmut<T, TX: Clone> {
    fn get_cfg(&self) -> TX;

    /// Sets a static module-level Cfg with a configuration info source and refresh mode.
    fn update_all(
        &self,
        cfg_src_fn: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    );

    fn update_refresh_mode(&self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode to the static module-level Cfg.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync;
}

pub trait CfgMut<T, TX: Clone> {
    /// Returns the configuration data in the cache, even if stale.
    fn get_cfg_cached(&self) -> TX;

    fn cache_expired(&self) -> bool;

    /// This will return the current configuration data, according to the object's cache refresh policy,
    /// with a possible change to cache state as a side-effect.
    fn get_cfg(&mut self) -> TX;

    fn replace(&mut self, other: Self);

    /// Updates the receiver with a configuration info source and refresh mode.
    fn update_all(
        &mut self,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    );

    fn update_refresh_mode(&mut self, refresh_mode: RefreshMode);

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode to the receiver.
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
    pub(crate) fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    ) -> Self {
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

    pub(crate) fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || g(&f());
        Self::new(src, refresh_mode)
    }
}

impl<T, TX> CfgMut<T, TX> for CfgRaw<T, TX>
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

impl<T, TX, I, IM> CfgInnerMut<T, TX, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgMut<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    // I don't understand why I have to do this as this method is defined in trait CfgImmut.
    pub fn get_cfg(&self) -> TX {
        CfgImmut::get_cfg(self)
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
        CfgInnerMut(IM::from(inner), PhantomData, PhantomData, PhantomData)
    }

    pub fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        factory: impl Fn(F, RefreshMode) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> T + Send + Sync,
    {
        Self::new_priv(factory(src, refresh_mode))
    }

    pub fn new_with_cfg_adapter_f<S, F, G>(
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        factory: impl Fn(F, G, RefreshMode) -> I,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_priv(factory(f, g, refresh_mode))
    }

    /// Updates the receiver with a configuration info source and refresh mode.
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
    /// sets it and the refresh mode to the receiver.
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

// impl<T, TX, I, IM> Clone for CfgInnerMut<T, TX, I, IM>
// where
//     TX: From<T> + Clone + core::fmt::Debug,
//     I: CfgDepsMut<T, TX> + Clone + core::fmt::Debug,
//     IM: InnerMutNc<I>,
// {
//     fn clone(&self) -> Self {
//         let inner = self.get_inner_clone();
//         Self::new_priv(inner)
//     }
// }

impl<T, TX, I, IM> CfgImmut<T, TX> for CfgInnerMut<T, TX, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgMut<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
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

    /// Updates the receiver with a configuration info source and refresh mode.
    fn update_all(&self, src: impl 'static + Fn() -> T + Send + Sync, refresh_mode: RefreshMode) {
        Self::update_all(self, src, refresh_mode)
    }

    fn update_refresh_mode(&self, refresh_mode: RefreshMode) {
        Self::update_refresh_mode(&self, refresh_mode)
    }

    /// Composes an application info source f with an adapter g for a particular module, then
    /// sets it and the refresh mode to the receiver.
    fn update_with_cfg_adapter<S, F, G>(&self, f: F, g: G, refresh_mode: RefreshMode)
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::update_with_cfg_adapter(&self, f, g, refresh_mode)
    }
}

// Type aliases for CfgDepsNc.

pub type CfgStd<T, TX, IM> = CfgInnerMut<T, TX, CfgRaw<T, TX>, IM>;

pub type CfgRefCell<T, TX> = CfgStd<T, TX, RefCell<CfgRaw<T, TX>>>;

pub type CfgArcSwap<T, TX> = CfgStd<T, TX, ArcSwap<CfgRaw<T, TX>>>;

pub type CfgRefCellRc<T> = CfgRefCell<T, Rc<T>>;

pub type CfgArcSwapRc<T> = CfgArcSwap<T, Rc<T>>;

pub type CfgRefCellArc<T> = CfgRefCell<T, Arc<T>>;

pub type CfgArcSwapArc<T> = CfgArcSwap<T, Arc<T>>;

pub type CfgRefCellId<T> = CfgRefCell<T, T>;

pub type CfgArcSwapId<T> = CfgArcSwap<T, T>;

// pub type CfgDefault<T> = CfgArcSwapArc<T>;
// pub type CfgDefault<T> = CfgRefCellArc<T>;
pub type CfgDefault<T> = CfgRefCellRc<T>;

impl<T, TX, IM> CfgStd<T, TX, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn new(src: impl 'static + Fn() -> T + Send + Sync, refresh_mode: RefreshMode) -> Self {
        Self::new_f(src, refresh_mode, CfgRaw::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, CfgRaw::new_with_cfg_adapter)
    }
}

impl<T, TX, IM> CfgStd<T, TX, IM>
where
    T: 'static + Clone + Send + Sync,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn new_with_const_or_cfg_adapter<S, F, G>(
        k: Option<&'static T>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        match k {
            Some(k) => {
                let src = move || k.clone();
                Self::new(src, refresh_mode)
            }
            None => Self::new_with_cfg_adapter(f, g, refresh_mode),
        }
    }
}
