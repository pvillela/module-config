use super::{set_once_cell, Cache, Cfg, InnerMut, RefreshMode, Src};
use std::sync::Arc;
use std::sync::OnceLock;

pub struct CfgOvd<T: 'static> {
    cfg_src: Option<Src<T>>,
    refresh_mode: Option<RefreshMode>,
}

impl<T> CfgOvd<T> {
    pub fn set_once_cell(
        cell: &OnceLock<CfgOvd<T>>,
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
        cell: &OnceLock<Self>,
        cfg_src: Src<T>,
        refresh_mode: RefreshMode,
    ) {
        let _ = set_once_cell(cell, Self::new(cfg_src, refresh_mode));
    }

    pub fn set_once_cell_with_cfg_adapter<S: 'static>(
        cell: &OnceLock<Self>,
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
    pub fn new_from_once_cell_def(cell: &OnceLock<CfgDef<T>>) -> Self {
        let CfgDef {
            cfg_src,
            refresh_mode,
        } = cell.get().expect("OnceLock not initialized");
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
}
