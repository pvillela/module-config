use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct CfgSrc<T: 'static> {
    src: Box<dyn 'static + Fn() -> T + Send + Sync>,
}

fn nil_cfg_src_fn<T: 'static>() -> T {
    panic!("Module used before being initialized");
}

impl<T: 'static> CfgSrc<T> {
    fn new(src: impl 'static + Fn() -> T + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
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

pub struct CfgSrcAdaptation<S: 'static, T: 'static> {
    pub target_src: &'static ArcSwap<CfgSrc<T>>,
    pub adapter: fn(&S) -> T,
}

pub fn set_adaptation_origin<S: 'static, T: 'static>(
    adaptation: &'static ArcSwap<CfgSrcAdaptation<S, T>>,
    origin_src: fn() -> Arc<S>,
) {
    let target_src = adaptation.load().target_src;
    let adapter = adaptation.load().adapter;
    target_src.store(Arc::new(CfgSrc::new(move || adapter(&(origin_src())))));
}
