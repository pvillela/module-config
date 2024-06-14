use arc_swap::ArcSwap;
use common::config::{get_app_configuration, AppCfgInfo};
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
        Self::new(move || adapter(&get_app_configuration()))
    }

    pub fn nil() -> Self {
        Self::new(nil_cfg_src_fn)
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
