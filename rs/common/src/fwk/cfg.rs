use super::compose_static_0;
use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

//=================
// Types

#[derive(Clone)]
pub struct AppCfg<T> {
    pub app_src: fn() -> T,
    pub refresh_mode: RefreshMode,
}

#[derive(Clone, Debug)]
pub enum RefreshMode {
    NoRefresh,
    Refreshable(Duration),
}

#[derive(Debug, Clone)]
pub struct Cache<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    last_refresh: Instant,
    value: TX,
    _t: PhantomData<T>,
}

pub trait InnerMut<I> {
    fn get_inner_clone(&self) -> I;

    fn set_inner(&self, inner: I);

    fn from(_: I) -> Self;

    fn with<V>(&self, _: impl Fn(&I) -> V) -> V;
}

/// A configuration source.
/// Does not publicly implement Clone on purpose.
pub enum Src<T: 'static> {
    Fn(fn() -> T),
    Ref(&'static (dyn Fn() -> T + Send + Sync)),
    Boxed(Arc<dyn Fn() -> T + Send + Sync>),
}

pub trait CfgImmut<T, TX: Clone> {
    fn get_cfg(&self) -> TX;

    fn get_src(&self) -> Src<T>;

    fn get_refresh_mode(&self) -> RefreshMode;
}

pub struct Cfg<T, TX, IM>
where
    T: 'static,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    src: Src<T>,
    refresh_mode: RefreshMode,
    icache: IM,
    _tx: PhantomData<TX>,
}

//=================
// InnerMut implementations

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

//=================
// Src implementations

impl<T> Src<T> {
    fn invoke(&self) -> T {
        match self {
            Self::Fn(src) => src(),
            Self::Ref(src) => src(),
            Self::Boxed(src) => src(),
        }
    }

    pub fn new_fn(f: fn() -> T) -> Self {
        Self::Fn(f)
    }

    pub fn new_ref(f: &'static (dyn Fn() -> T + Send + Sync)) -> Self {
        Self::Ref(f)
    }

    pub fn new_boxed(f: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self::Boxed(Arc::new(f))
    }

    pub fn new_ref_with_cfg_adapter<S: 'static>(f: fn() -> S, g: fn(&S) -> T) -> Self {
        Src::new_ref(compose_static_0(f, g))
    }

    pub fn new_boxed_with_cfg_adapter<S: 'static>(f: fn() -> S, g: fn(&S) -> T) -> Self {
        Src::new_boxed(move || g(&f()))
    }

    pub(crate) fn clone(&self) -> Self {
        match self {
            Self::Fn(src) => Self::Fn(src.clone()),
            Self::Ref(src) => Self::Ref(*src),
            Self::Boxed(src) => Self::Boxed(src.clone()),
        }
    }
}

//=================
// Cache implementations

impl<T, TX> Cache<T, TX>
where
    TX: From<T> + Clone + core::fmt::Debug,
{
    pub(crate) fn new(cfg: T) -> Self {
        Cache {
            last_refresh: Instant::now(),
            value: cfg.into(),
            _t: PhantomData,
        }
    }

    fn refresh(&mut self, src: &Src<T>) {
        let cfg: TX = src.invoke().into();
        self.value = cfg;
    }

    pub(crate) fn new_with_src(src: &Src<T>) -> Self {
        let cfg = src.invoke();
        Self::new(cfg)
    }

    pub(crate) fn get_cfg_cached(&self) -> TX {
        self.value.clone()
    }

    pub(crate) fn cache_expired(&self, refresh_mode: RefreshMode) -> bool {
        // println!("refresh_mode={:?}", self.refresh_mode);
        let res = match refresh_mode {
            RefreshMode::NoRefresh => false,
            RefreshMode::Refreshable(cache_ttl) => {
                // println!(
                //     "cache.last_refresh.elapsed()={:?}, cache_ttl={:?}",
                //     self.last_refresh.elapsed(),
                //     cache_ttl
                // );
                if self.last_refresh.elapsed() > cache_ttl {
                    true
                } else {
                    false
                }
            }
        };
        // println!("cache_expired={}", res);
        res
    }

    pub(crate) fn get_cfg(&mut self, src: &Src<T>, refresh_mode: RefreshMode) -> TX {
        if self.cache_expired(refresh_mode) {
            self.refresh(src);
        }
        self.value.clone()
    }
}

//=================
// Cfg implementations

impl<T, TX, IM> Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    // I don't understand why I have to do this as this method is defined in trait CfgImmut.
    pub fn get_cfg(&self) -> TX {
        CfgImmut::get_cfg(self)
    }

    fn get_inner(&self) -> &IM {
        &self.icache
    }

    fn get_inner_clone(&self) -> Cache<T, TX> {
        self.icache.get_inner_clone()
    }

    fn set_inner(&self, inner: Cache<T, TX>) {
        self.icache.set_inner(inner);
    }

    fn new_priv(src: Src<T>, refresh_mode: RefreshMode, cache: Cache<T, TX>) -> Self {
        Cfg {
            src,
            refresh_mode,
            icache: IM::from(cache),
            _tx: PhantomData,
        }
    }

    pub fn new(src: Src<T>, refresh_mode: RefreshMode) -> Self {
        println!("Cfg::new with refresh_mode={:?}", refresh_mode);
        let cache = Cache::new_with_src(&src);
        Self::new_priv(src, refresh_mode, cache)
    }

    pub fn new_ref_with_cfg_adapter<S: 'static>(
        f: fn() -> S,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let src = Src::new_ref_with_cfg_adapter(f, g);
        let cache = Cache::new_with_src(&src);
        Self::new_priv(src, refresh_mode, cache)
    }

    pub fn new_boxed_with_cfg_adapter<S: 'static>(
        f: fn() -> S,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let src = Src::new_boxed_with_cfg_adapter(f, g);
        let cache = Cache::new_with_src(&src);
        Self::new_priv(src, refresh_mode, cache)
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    T: 'static + Clone + Send + Sync,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    pub fn new_ref_with_const_or_cfg_adapter<S: 'static>(
        k: Option<&'static T>,
        f: fn() -> S,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        match k {
            Some(k) => {
                let src = Src::new_boxed(move || k.clone());
                Self::new(src, refresh_mode)
            }
            None => Self::new_ref_with_cfg_adapter(f, g, refresh_mode),
        }
    }

    pub fn new_boxed_with_const_or_cfg_adapter<S: 'static>(
        k: Option<&'static T>,
        f: fn() -> S,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        // println!(
        //     "Cfg::new_boxed_with_const_or_cfg_adapter with refresh_mode={:?}",
        //     refresh_mode
        // );
        match k {
            Some(k) => {
                let src = Src::new_boxed(move || k.clone());
                Self::new(src, refresh_mode)
            }
            None => Self::new_boxed_with_cfg_adapter(f, g, refresh_mode),
        }
    }
}

impl<T, TX, IM> CfgImmut<T, TX> for Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    fn get_cfg(&self) -> TX {
        let inner = self.get_inner();

        let f_cache_expired =
            move |i: &Cache<T, TX>| -> bool { i.cache_expired(self.refresh_mode.clone()) };

        let f_cfg_cached = move |i: &Cache<T, TX>| -> TX { i.get_cfg_cached() };

        let cache_expired = inner.with(f_cache_expired);

        if cache_expired {
            let mut inner = self.get_inner_clone();
            let cfg = inner.get_cfg(&self.src, self.refresh_mode.clone());
            self.set_inner(inner);
            cfg
        } else {
            inner.with(f_cfg_cached)
        }
    }

    fn get_refresh_mode(&self) -> RefreshMode {
        self.refresh_mode.clone()
    }

    fn get_src(&self) -> Src<T> {
        self.src.clone()
    }
}

//=================
// Type aliases for Cfg

pub type CfgRefCell<T, TX> = Cfg<T, TX, RefCell<Cache<T, TX>>>;

pub type CfgArcSwap<T, TX> = Cfg<T, TX, ArcSwap<Cache<T, TX>>>;

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
