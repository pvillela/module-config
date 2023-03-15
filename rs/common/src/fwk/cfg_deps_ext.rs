use super::{CfgDeps, CfgDepsInnerMut, CfgDepsMut, CfgDepsRaw, InnerMut, RefreshMode};
use once_cell::sync::OnceCell;
use std::sync::Arc;

impl<T, TX, U, I, IM> CfgDepsInnerMut<T, TX, U, I, IM>
where
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    I: CfgDepsMut<T, TX, U> + Clone + core::fmt::Debug,
    IM: InnerMut<I>,
{
    pub fn get_from_once_cell(cell: &OnceCell<Self>) -> (TX, U) {
        cell.get().expect("OnceCell not initialized").get_cfg_deps()
    }
}

impl<T, TX, U, IM> CfgDeps<T, TX, U, IM>
where
    T: Clone,
    TX: From<T> + Clone + core::fmt::Debug,
    U: Clone,
    IM: InnerMut<CfgDepsRaw<T, TX, U>>,
{
    pub fn set(
        cell: &OnceCell<Self>,
        src: impl 'static + Fn() -> T + Send + Sync,
        refresh_mode: RefreshMode,
        deps: U,
    ) {
        let res = cell.set(Self::new(src, refresh_mode, deps));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }

    pub fn set_with_cfg_adapter<S, F, G>(
        cell: &OnceCell<Self>,
        f: F,
        g: G,
        refresh_mode: RefreshMode,
        deps: U,
    ) where
        F: 'static + Fn() -> Arc<S> + Send + Sync,
        G: 'static + Fn(&S) -> T + Send + Sync,
    {
        let res = cell.set(Self::new_with_cfg_adapter(f, g, refresh_mode, deps));
        if let Err(_) = res {
            println!("OnceCell already initialized");
        }
    }
}
