use crate::config::app_cfg_info::{get_app_configuration, AppCfgInfo};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct CfgSrc<T: 'static> {
    src: Box<dyn 'static + Fn() -> T + Send + Sync>,
}

fn nil_cfg_src_fn<T: 'static>() -> T {
    panic!("Module used before being initialized");
}

impl<T: 'static> CfgSrc<T> {
    pub fn new(src: impl 'static + Fn() -> T + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
    }

    pub fn from_adapter(adapter: fn(&AppCfgInfo) -> T) -> Self {
        Self::new(move || adapter(get_app_configuration().as_ref()))
    }

    pub fn nil() -> Self {
        Self::new(nil_cfg_src_fn)
    }

    pub fn set_src(&mut self, src: impl 'static + Fn() -> T + Send + Sync) {
        self.src = Box::new(src);
    }

    pub fn get(&self) -> T {
        self.src.as_ref()()
    }
}

pub fn update_cfg_src_with_fn<T: 'static>(
    cfg_src_static: &ArcSwap<CfgSrc<T>>,
    cfg_src_fn: impl 'static + Fn() -> T + Send + Sync,
) {
    cfg_src_static.store(Arc::new(CfgSrc::new(cfg_src_fn)));
}

pub fn update_cfg_src_with_adapter<T: 'static>(
    cfg_src_static: &ArcSwap<CfgSrc<T>>,
    adapter: fn(&AppCfgInfo) -> T,
) {
    update_cfg_src_with_fn(cfg_src_static, move || {
        adapter(get_app_configuration().as_ref())
    });
}

pub const fn nil_cfg_src<T: 'static>() -> Lazy<ArcSwap<CfgSrc<T>>> {
    Lazy::new(|| ArcSwap::from_pointee(CfgSrc::nil()))
}

pub type CfgSrcAdapter<S, T> = fn(S) -> T;

pub struct CfgSrcAdaptation<S: 'static, T: 'static> {
    targetSrc: CfgSrc<T>,
    adapter: fn(&S) -> T,
}

impl<S: 'static, T: 'static> CfgSrcAdaptation<S, T> {
    fn setOrigin(&mut self, originSrc: fn() -> S) {
        let adapter = self.adapter.clone();
        self.targetSrc.set_src(move || adapter(&(originSrc())))
    }
}
