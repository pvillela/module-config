use super::{CfgInnerMut, CfgMut, CfgRaw, CfgStd, InnerMut, RefreshMode};
use once_cell::sync::OnceCell;
use std::sync::Arc;

impl<T, TX, I, IM> CfgInnerMut<T, TX, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    I: CfgMut<T, TX> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    pub fn get_from_once_cell(cell: &OnceCell<Self>) -> TX {
        cell.get().expect("OnceCell not initialized").get_cfg()
    }
}

impl<T, TX, IM> CfgStd<T, TX, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn set(
        cell: &OnceCell<Self>,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
    ) {
        let res = cell.set(Self::new(src, refresh_mode));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }

    pub fn set_with_cfg_adapter<S, F, G>(
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

impl<T, TX, IM> CfgStd<T, TX, IM>
where
    T: 'static + Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    IM: InnerMut<CfgRaw<T, TX>>,
{
    pub fn new_with_override<S: 'static>(
        ovr: Option<&CfgOvd<T>>,
        f: fn() -> Arc<S>,
        g: fn(&S) -> T,
        refresh_mode: RefreshMode,
    ) -> Self {
        let ov = match ovr {
            Some(ov) => CfgOvd {
                cfg_src: ov.cfg_src,
                refresh_mode: ov.refresh_mode.clone(),
            },
            None => CfgOvd {
                cfg_src: None,
                refresh_mode: None,
            },
        };
        if ov.cfg_src == None {
            Self::new_with_cfg_adapter(f, g, ov.refresh_mode.unwrap_or(refresh_mode))
        } else {
            Self::new(ov.cfg_src.unwrap(), ov.refresh_mode.unwrap_or(refresh_mode))
        }
    }
}
