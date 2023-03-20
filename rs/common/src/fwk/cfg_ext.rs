use super::{
    compose_static_0_arc, set_once_cell, static_closure_0_thread_safe, Cfg, CfgImmut, CfgRaw,
    InnerMut, RefreshMode,
};
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
        src: &'static (dyn Fn() -> T + Send + Sync),
        refresh_mode: RefreshMode,
    ) {
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
        let res = cell.set(Self::new_with_cfg_adapter(f, g, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }
}

pub struct CfgOvd<T: 'static> {
    cfg_src: Option<&'static (dyn Fn() -> T + Send + Sync)>,
    refresh_mode: Option<RefreshMode>,
}

// TODO: REMOVE BELOW.
// impl<T: 'static + Send + Sync> CfgOvd<T> {
//     pub fn set_once_cell_with_function(
//         cell: &OnceCell<Self>,
//         cfg_src: Option<fn() -> T>,
//         refresh_mode: Option<RefreshMode>,
//     ) {
//         if cell.get().is_some() {
//             println!("OnceCell already initialized");
//             return;
//         }

//         let f = cfg_src.unwrap();
//         // let cfg_src =
//         let cfg_src = Box::leak(Box::new(move || f()));
//         // let cfg_src = cfg_src.unwrap();
//         // let cfg_src = static_closure_0_thread_safe(f);
//         let res = cell.set(CfgOvd {
//             cfg_src: Some(cfg_src),
//             refresh_mode,
//         });
//         if let Err(_) = res {
//             println!("OnceCell already initialized");
//         }
//     }
// }

impl<T> CfgOvd<T> {
    pub fn set_once_cell(
        cell: &OnceCell<CfgOvd<T>>,
        cfg_src: Option<&'static (dyn Fn() -> T + Send + Sync)>,
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
    cfg_src: &'static (dyn Fn() -> T + Send + Sync),
    refresh_mode: RefreshMode,
}

impl<T> CfgDef<T> {
    pub fn new(cfg_src: &'static (dyn Fn() -> T + Send + Sync), refresh_mode: RefreshMode) -> Self {
        CfgDef {
            cfg_src,
            refresh_mode,
        }
    }

    pub fn new_with_cfg_src(
        cfg_src: impl Fn() -> T + Send + Sync + 'static,
        refresh_mode: RefreshMode,
    ) -> Self {
        CfgDef {
            cfg_src: static_closure_0_thread_safe(cfg_src),
            refresh_mode,
        }
    }

    pub fn new_with_cfg_adapter<S: 'static>(
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let src = compose_static_0_arc(f, g);
        Self::new(src, refresh_mode)
    }

    pub fn set_once_cell_with_cfg_src(
        cell: &OnceCell<Self>,
        cfg_src: impl Fn() -> T + Send + Sync + 'static,
        refresh_mode: RefreshMode,
    ) {
        let _ = set_once_cell(cell, Self::new_with_cfg_src(cfg_src, refresh_mode));
    }

    pub fn set_once_cell_with_cfg_adapter<S: 'static>(
        cell: &OnceCell<Self>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) {
        let _ = set_once_cell(cell, Self::new_with_cfg_adapter(f, g, refresh_mode));
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
        if ovd.cfg_src.is_none() {
            Self::new_with_cfg_adapter(f, g, ovd.refresh_mode.unwrap_or(refresh_mode))
        } else {
            let src = ovd.cfg_src.unwrap();
            Self::new(src, ovd.refresh_mode.unwrap_or(refresh_mode))
        }
    }

    pub fn new_from_static<C, TX1>(cfg_opt: Option<&'static C>) -> Self
    where
        TX1: Clone,
        C: CfgImmut<T, TX1>,
    {
        if let Some(cfg) = cfg_opt {
            Self::new(cfg.get_cfg_src(), cfg.get_refresh_mode())
        } else {
            panic!("Configuration not initialized.")
        }
    }

    pub fn new_from_def(def_opt: Option<&'static CfgDef<T>>) -> Self {
        if let Some(def) = def_opt {
            Self::new(def.cfg_src, def.refresh_mode.clone())
        } else {
            panic!("Configuration not initialized.")
        }
    }
}
