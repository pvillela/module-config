use super::{
    get_from_once_cell, get_initialized_option, Cache, Cfg, CfgArcSwapArc, CfgImmut, CfgRefCellRc,
    InnerMut, RefreshMode, Src,
};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Arc;

impl<T, TX, IM> Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    pub fn get_from_once_cell(cell: &OnceCell<Self>) -> TX {
        cell.get().expect("OnceCell not initialized").get_cfg()
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    pub fn set_once_cell(cell: &OnceCell<Self>, src: Src<T>, refresh_mode: RefreshMode) {
        let res = cell.set(Self::new(src, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }

    pub fn set_once_cell_with_cfg_adapter<S: 'static>(
        cell: &OnceCell<Self>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) {
        let res = cell.set(Self::new_ref_with_cfg_adapter(f, g, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }
}

pub fn cfg_lazy_to_thread_local<T: Clone + core::fmt::Debug>(
    cfg: &Lazy<CfgArcSwapArc<T>>,
) -> CfgRefCellRc<T> {
    let src = cfg.get_src();
    let refresh_mode = cfg.get_refresh_mode();
    CfgRefCellRc::new(src, refresh_mode)
}

pub fn cfg_once_cell_to_thread_local<T: Clone + core::fmt::Debug>(
    cfg: &OnceCell<CfgArcSwapArc<T>>,
) -> CfgRefCellRc<T> {
    let cfg = get_from_once_cell(cfg);
    let src = cfg.get_src();
    let refresh_mode = cfg.get_refresh_mode();
    CfgRefCellRc::new(src, refresh_mode)
}

pub fn cfg_global_to_thread_local<T, TX, IM>(cfg: &Option<Cfg<T, TX, IM>>) -> CfgRefCellRc<T>
where
    T: Clone + Send + Sync + core::fmt::Debug,
    TX: From<T> + Clone + Sync + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>> + Sync,
{
    let cfg = get_initialized_option(cfg);
    let src = cfg.get_src();
    let refresh_mode = cfg.get_refresh_mode();
    CfgRefCellRc::new(src, refresh_mode)
}
