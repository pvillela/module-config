use core::panic;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::Arc;

pub struct CfgSrc<T: 'static> {
    src: Box<dyn 'static + Fn() -> Arc<T> + Send + Sync>,
}

impl<T: 'static> CfgSrc<T> {
    fn new(src: impl 'static + Fn() -> Arc<T> + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
    }

    pub fn nil() -> Self {
        Self::new(nil_cfg_src_fn)
    }

    pub fn get(&self) -> Arc<T> {
        self.src.as_ref()()
    }
}

fn nil_cfg_src_fn<T: 'static>() -> T {
    panic!("Module used before being initialized");
}

pub fn update_cfg_src_with_fn<T: 'static>(
    cfg_src_static: &OnceCell<CfgSrc<T>>,
    cfg_src_fn: impl 'static + Fn() -> Arc<T> + Send + Sync,
) {
    if let Err(_) = cfg_src_static.set(CfgSrc::new(cfg_src_fn)) {
        panic!("OnceCell already initialized");
    };
}

pub enum ArcCache<T> {
    NoCache,
    EmptyCache,
    Value(Arc<T>),
}

/// Composes an application info source f with an adapter g for a particular module, then
/// sets the static module config source.
pub fn adapt_by_ref<S, T: Clone + Send + Sync, F, G>(
    f: F,
    g: G,
    cache_ref: &mut ArcCache<T>,
    mod_cfg_src: &OnceCell<CfgSrc<T>>,
) where
    F: 'static + Fn() -> Arc<S> + Send + Sync,
    G: 'static + Fn(&S) -> T + Send + Sync,
{
    let v = if let ArcCache::Value(v) = cache_ref {
        v.clone()
    } else {
        let v = Arc::new(g(f().deref()));
        if let ArcCache::EmptyCache = cache_ref {
            *cache_ref = ArcCache::Value(v.clone())
        }
        v
    };

    let h = move || v.clone();
    if let Err(_) = mod_cfg_src.set(CfgSrc { src: Box::new(h) }) {
        panic!("OnceCell already initialized");
    };
}
