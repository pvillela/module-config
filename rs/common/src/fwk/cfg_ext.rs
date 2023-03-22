use super::{set_once_cell, Cache, Cfg, InnerMut, RefreshMode, Src};
use once_cell::sync::OnceCell;
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

pub struct CfgOvd<T: 'static> {
    cfg_src: Option<Src<T>>,
    refresh_mode: Option<RefreshMode>,
}

impl<T> CfgOvd<T> {
    pub fn set_once_cell(
        cell: &OnceCell<CfgOvd<T>>,
        cfg_src: Option<Src<T>>,
        refresh_mode: Option<RefreshMode>,
    ) -> Result<(), Self> {
        set_once_cell(
            cell,
            CfgOvd {
                cfg_src,
                refresh_mode,
            },
        )
    }
}

pub struct CfgDef<T: 'static> {
    cfg_src: Src<T>,
    refresh_mode: RefreshMode,
}

impl<T> CfgDef<T> {
    pub fn new(cfg_src: Src<T>, refresh_mode: RefreshMode) -> Self {
        CfgDef {
            cfg_src,
            refresh_mode,
        }
    }

    pub fn new_ref_with_cfg_adapter<S: 'static>(
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let src = Src::new_ref_with_cfg_adapter(f, g);
        Self::new(src, refresh_mode)
    }

    pub fn set_once_cell_with_cfg_src(
        cell: &OnceCell<Self>,
        cfg_src: Src<T>,
        refresh_mode: RefreshMode,
    ) {
        let _ = set_once_cell(cell, Self::new(cfg_src, refresh_mode));
    }

    pub fn set_once_cell_with_cfg_adapter<S: 'static>(
        cell: &OnceCell<Self>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) {
        let _ = set_once_cell(cell, Self::new_ref_with_cfg_adapter(f, g, refresh_mode));
    }
}

impl<T, TX, IM> Cfg<T, TX, IM>
where
    T: 'static + Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<Cache<T, TX>>,
{
    pub fn new_from_once_cell_def(cell: &OnceCell<CfgDef<T>>) -> Self {
        let CfgDef {
            cfg_src,
            refresh_mode,
        } = cell.get().expect("OnceCell not initialized");
        Self::new(cfg_src.clone(), refresh_mode.clone())
    }

    pub fn new_ref_with_cfg_adapter_and_override<S: 'static>(
        ovd: Option<&CfgOvd<T>>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        if ovd.is_none() {
            return Self::new_ref_with_cfg_adapter(f, g, refresh_mode);
        };

        let ovd = ovd.unwrap();
        let refresh_mode = ovd.refresh_mode.clone().unwrap_or(refresh_mode);
        match &ovd.cfg_src {
            None => Self::new_ref_with_cfg_adapter(f, g, refresh_mode),
            Some(src) => {
                let src = src.clone();
                Self::new(src, refresh_mode)
            }
        }
    }

    pub fn new_boxed_with_cfg_adapter_and_override<S: 'static>(
        ovd: Option<&CfgOvd<T>>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        if ovd.is_none() {
            return Self::new_boxed_with_cfg_adapter(f, g, refresh_mode);
        };

        let ovd = ovd.unwrap();
        let refresh_mode = ovd.refresh_mode.clone().unwrap_or(refresh_mode);
        match &ovd.cfg_src {
            None => Self::new_boxed_with_cfg_adapter(f, g, refresh_mode),
            Some(src) => {
                let src = src.clone();
                Self::new(src, refresh_mode)
            }
        }
    }

    // pub fn new_from_def(def_opt: Option<&'static CfgDef<T>>) -> Self {
    //     if let Some(def) = def_opt {
    //         Self::new(def.cfg_src, def.refresh_mode.clone())
    //     } else {
    //         panic!("Configuration not initialized.")
    //     }
    // }
}
