use super::{Cfg, CfgImmut, CfgRaw, InnerMut, RefreshMode};
use once_cell::sync::OnceCell;
use std::sync::Arc;

impl<T, TX, IM> Cfg<T, TX, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn get_from_once_cell(cell: &OnceCell<Self>) -> TX {
        cell.get().expect("OnceCell not initialized").get_cfg()
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn set_once_cell(
        cell: &OnceCell<Self>,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    ) {
        let res = cell.set(Self::new(src, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }

    pub fn set_once_cell_with_cfg_adapter<S, F, G>(
        cell: &OnceCell<Self>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
    ) where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let res = cell.set(Self::new_with_cfg_adapter(f, g, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }
}

pub struct CfgOvd<T> {
    pub cfg_src: Option<fn() -> T>,
    pub refresh_mode: Option<RefreshMode>,
}

pub struct CfgDef<T> {
    cfg_src: Box<dyn Fn() -> T + Send + Sync>,
    refresh_mode: RefreshMode,
}

impl<T> CfgDef<T> {
    pub fn new(cfg_src: impl 'static + Fn() -> T + Send + Sync, refresh_mode: RefreshMode) -> Self {
        CfgDef {
            cfg_src: Box::new(cfg_src),
            refresh_mode,
        }
    }

    pub fn new_with_cfg_adapter<S, F, G>(f: F, g: G, refresh_mode: RefreshMode) -> Self
    where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let src = move || g(&f());
        Self::new(src, refresh_mode)
    }

    pub fn set_once_cell_with_cfg_adapter<S, F, G>(
        cell: &OnceCell<Self>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
    ) where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let res = cell.set(Self::new_with_cfg_adapter(f, g, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    T: 'static + Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn new_with_override<S: 'static>(
        ovd: Option<&CfgOvd<T>>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let ovd = match ovd {
            Some(ovd) => CfgOvd {
                cfg_src: ovd.cfg_src,
                refresh_mode: ovd.refresh_mode.clone(),
            },
            None => CfgOvd {
                cfg_src: None,
                refresh_mode: None,
            },
        };
        if ovd.cfg_src == None {
            Self::new_with_cfg_adapter(f, g, ovd.refresh_mode.unwrap_or(refresh_mode))
        } else {
            Self::new(
                ovd.cfg_src.unwrap(),
                ovd.refresh_mode.unwrap_or(refresh_mode),
            )
        }
    }

    pub fn new_from_static<C, TX1>(cfg_opt: Option<&'static C>) -> Self
    where
        TX1: Clone,
        C: CfgImmut<T, TX1> + Send + Sync,
    {
        if let Some(cfg) = cfg_opt {
            Self::new(|| cfg.get_cfg_src()(), cfg.get_refresh_mode())
        } else {
            panic!("Configuration not initialized.")
        }
    }

    pub fn new_from_def(def_opt: Option<&'static CfgDef<T>>) -> Self {
        if let Some(def) = def_opt {
            Self::new(|| (def.cfg_src)(), def.refresh_mode.clone())
        } else {
            panic!("Configuration not initialized.")
        }
    }
}
