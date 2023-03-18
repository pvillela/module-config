use super::{CfgArcSwap, CfgImmut, CfgRaw, CfgRefCell, CfgStd, InnerMut, RefreshMode};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

pub struct CfgDeps<T, TX, C, U>
where
    TX: Clone,
    C: CfgImmut<T, TX>,
{
    cfg: C,
    deps: U,
    _t: PhantomData<T>,
    _tx: PhantomData<TX>,
}

impl<T, TX, C, U> CfgDeps<T, TX, C, U>
where
    TX: Clone,
    C: CfgImmut<T, TX>,
    U: Clone,
{
    pub fn get_cfg(&self) -> TX {
        self.cfg.get_cfg()
    }

    pub fn get_deps(&self) -> U {
        self.deps.clone()
    }

    fn new_priv(cfg: C, deps: U) -> Self {
        Self {
            cfg,
            deps,
            _t: PhantomData,
            _tx: PhantomData,
        }
    }

    fn new_f<F>(
        src: F,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(F, RefreshMode) -> C,
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
        factory: impl Fn(F, G, RefreshMode) -> C,
    ) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_priv(factory(f, g, refresh_mode), deps)
    }
}

// Type aliases for CfgDeps.

pub type CfgDepsStd<T, TX, IM, U> = CfgDeps<T, TX, CfgStd<T, TX, IM>, U>;

pub type CfgDepsRefCell<T, TX, U> = CfgDepsStd<T, TX, CfgRefCell<T, TX>, U>;

pub type CfgDepsArcSwap<T, TX, U> = CfgDepsStd<T, TX, CfgArcSwap<T, TX>, U>;

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

impl<T, TX, IM, U> CfgDepsStd<T, TX, IM, U>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
    U: Clone,
{
    pub fn new(
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_f(src, refresh_mode, deps, CfgStd::new)
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode, deps: U) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, CfgStd::new_with_cfg_adapter)
    }
}

impl<T, TX, IM, U> CfgDepsStd<T, TX, IM, U>
where
    T: 'static + Clone + Send + Sync,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
    U: Clone,
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
