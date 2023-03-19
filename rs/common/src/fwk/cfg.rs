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

pub struct Cfg<T, TX, IM>(IM, PhantomData<T>, PhantomData<TX>)
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>;

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

    fn get_cfg_src(&self) -> Arc<dyn 'static + Fn() -> T + Send + Sync>;

    fn get_refresh_mode(&self) -> RefreshMode;
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

    pub(crate) fn get_cfg_cached(&self) -> TX {
        self.cache.value.clone()
    }

    pub(crate) fn cache_expired(&self) -> bool {
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

    pub(crate) fn get_cfg(&mut self) -> TX {
        if self.cache_expired() {
            self.refresh();
        }
        self.cache.value.clone()
    }

    pub(crate) fn get_cfg_src(&self) -> Arc<dyn 'static + Fn() -> T + Send + Sync> {
        self.src.clone()
    }

    pub(crate) fn get_refresh_mode(&self) -> RefreshMode {
        self.refresh_mode.clone()
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    // I don't understand why I have to do this as this method is defined in trait CfgImmut.
    pub fn get_cfg(&self) -> TX {
        CfgImmut::get_cfg(self)
    }

    fn get_inner(&self) -> &IM {
        &self.0
    }

    fn get_inner_clone(&self) -> CfgRaw<T, TX> {
        self.0.get_inner_clone()
    }

    fn set_inner(&self, inner: CfgRaw<T, TX>) {
        self.0.set_inner(inner);
    }

    fn new_priv(inner: CfgRaw<T, TX>) -> Self {
        Cfg(IM::from(inner), PhantomData, PhantomData)
    }

    pub fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        factory: impl Fn(F, RefreshMode) -> CfgRaw<T, TX>,
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
        factory: impl Fn(F, G, RefreshMode) -> CfgRaw<T, TX>,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_priv(factory(f, g, refresh_mode))
    }
}

impl<T, TX, IM> CfgImmut<T, TX> for Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    fn get_cfg(&self) -> TX {
        let inner = self.get_inner();

        let f_cache_expired = move |i: &CfgRaw<T, TX>| -> bool { i.cache_expired() };

        let f_cfg_cached = move |i: &CfgRaw<T, TX>| -> TX { i.get_cfg_cached() };

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

    fn get_cfg_src(&self) -> Arc<dyn 'static + Fn() -> T + Send + Sync> {
        let f = move |i: &CfgRaw<T, TX>| i.get_cfg_src();
        self.get_inner().with(f)
    }

    fn get_refresh_mode(&self) -> RefreshMode {
        let f = move |i: &CfgRaw<T, TX>| i.get_refresh_mode();
        self.get_inner().with(f)
    }
}

// Type aliases for CfgDepsNc.

pub type CfgRefCell<T, TX> = Cfg<T, TX, RefCell<CfgRaw<T, TX>>>;

pub type CfgArcSwap<T, TX> = Cfg<T, TX, ArcSwap<CfgRaw<T, TX>>>;

pub type CfgRefCellRc<T> = CfgRefCell<T, Rc<T>>;

pub type CfgArcSwapRc<T> = CfgArcSwap<T, Rc<T>>;

pub type CfgRefCellArc<T> = CfgRefCell<T, Arc<T>>;

pub type CfgArcSwapArc<T> = CfgArcSwap<T, Arc<T>>;
pub type CfgArc<T> = CfgArcSwapArc<T>;

pub type CfgRefCellId<T> = CfgRefCell<T, T>;

pub type CfgArcSwapId<T> = CfgArcSwap<T, T>;

// pub type CfgDefault<T> = CfgArcSwapArc<T>;
// pub type CfgDefault<T> = CfgRefCellArc<T>;
pub type CfgDefault<T> = CfgRefCellRc<T>;

impl<T, TX, IM> Cfg<T, TX, IM>
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

impl<T, TX, IM> Cfg<T, TX, IM>
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
