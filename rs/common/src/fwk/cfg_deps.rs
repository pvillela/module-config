use super::{
    Cache, Cfg, CfgArcSwapArc, CfgArcSwapId, CfgArcSwapRc, CfgImmut, CfgRefCellArc, CfgRefCellId,
    CfgRefCellRc, InnerMut, RefreshMode, Src,
};
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

    fn new_f(
        src: Src<T>,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(Src<T>, RefreshMode) -> C,
    ) -> Self {
        Self::new_priv(factory(src, refresh_mode), deps)
    }

    pub fn new_with_cfg_adapter_f<S>(
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(fn() -> Arc<S>, fn(&S) -> T, RefreshMode) -> C,
    ) -> Self {
        Self::new_priv(factory(f, g, refresh_mode), deps)
    }

    pub fn new_with_const_or_cfg_adapter_f<S>(
        k: Option<&'static T>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
        factory: impl Fn(Option<&'static T>, fn() -> Arc<S>, fn(&S) -> T, RefreshMode) -> C,
    ) -> Self {
        Self::new_priv(factory(k, f, g, refresh_mode), deps)
    }
}

///////////////////
// Type aliases for CfgDeps.

pub type CfgDepsStd<T, TX, IM, U> = CfgDeps<T, TX, Cfg<T, TX, IM>, U>;

pub type CfgDepsRefCellRc<T, U> = CfgDeps<T, Rc<T>, CfgRefCellRc<T>, U>;

pub type CfgDepsArcSwapRc<T, U> = CfgDeps<T, Rc<T>, CfgArcSwapRc<T>, U>;

pub type CfgDepsRefCellArc<T, U> = CfgDeps<T, Arc<T>, CfgRefCellArc<T>, U>;

pub type CfgDepsArcSwapArc<T, U> = CfgDeps<T, Arc<T>, CfgArcSwapArc<T>, U>;

pub type CfgDepsRefCellId<T, U> = CfgDeps<T, T, CfgRefCellId<T>, U>;

pub type CfgDepsArcSwapId<T, U> = CfgDeps<T, T, CfgArcSwapId<T>, U>;

pub type CfgDepsArc<T, U> = CfgDepsArcSwapArc<T, U>;

// pub type CfgDepsDefault<T, U> = CfgDepsArcSwapArc<T, U>;
// pub type CfgDepsDefault<T, U> = CfgDepsRefCellArc<T, U>;
pub type CfgDepsDefault<T, U> = CfgDepsRefCellRc<T, U>;

///////////////////
// Factory methods for CfgDepsStd.

impl<T, TX, IM, U> CfgDepsStd<T, TX, IM, U>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
    U: Clone,
{
    pub fn new(src: Src<T>, refresh_mode: RefreshMode, deps: U) -> Self {
        Self::new_f(src, refresh_mode, deps, Cfg::new)
    }

    pub fn new_ref_with_cfg_adapter<S: 'static>(
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, Cfg::new_ref_with_cfg_adapter)
    }

    pub fn new_boxed_with_cfg_adapter<S: 'static>(
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_with_cfg_adapter_f(f, g, refresh_mode, deps, Cfg::new_boxed_with_cfg_adapter)
    }
}

impl<T, TX, IM, U> CfgDepsStd<T, TX, IM, U>
where
    T: 'static + Clone + Send + Sync,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
    U: Clone,
{
    pub fn new_ref_with_const_or_cfg_adapter<S: 'static>(
        k: Option<&'static T>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_with_const_or_cfg_adapter_f(
            k,
            f,
            g,
            refresh_mode,
            deps,
            Cfg::new_ref_with_const_or_cfg_adapter,
        )
    }

    pub fn new_boxed_with_const_or_cfg_adapter<S: 'static>(
        k: Option<&'static T>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
        deps: U,
    ) -> Self {
        Self::new_with_const_or_cfg_adapter_f(
            k,
            f,
            g,
            refresh_mode,
            deps,
            Cfg::new_boxed_with_const_or_cfg_adapter,
        )
    }
}
