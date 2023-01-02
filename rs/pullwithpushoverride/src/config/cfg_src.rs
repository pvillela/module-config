use crate::config::app_cfg_info::{getAppConfiguration, AppCfgInfo};
use std::{cell::RefCell, sync::Arc};

fn nilCfgSrc<T>() -> Arc<T> {
    panic!("Module used before being initialized");
}

pub struct CfgSrc<'a, T: 'a> {
    src: Box<dyn 'a + Fn() -> Arc<T> + Send + Sync>,
}

impl<'a, T: 'a> CfgSrc<'a, T> {
    pub fn new(src: impl 'a + Fn() -> Arc<T> + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
    }

    pub fn from_adapter(adapter: Option<fn(&AppCfgInfo) -> Arc<T>>) -> CfgSrc<'a, T> {
        makeCfgSrc(adapter)
    }

    pub fn set_src(&mut self, src: impl 'a + Fn() -> Arc<T> + Send + Sync) {
        self.src = Box::new(src);
    }

    pub fn get(&self) -> Arc<T> {
        self.src.as_ref()()
    }
}

pub fn makeCfgSrc0<'a, T: 'a>(
    adapter: Option<fn(&AppCfgInfo) -> Arc<T>>,
) -> RefCell<Box<dyn 'a + Fn() -> Arc<T>>> {
    if let Some(adapter) = adapter {
        // let x = adapter(getAppConfiguration().as_ref());
        RefCell::new(Box::new(move || adapter(getAppConfiguration().as_ref())))
    } else {
        RefCell::new(Box::new(nilCfgSrc))
    }
}

pub fn makeCfgSrc1<'a, T: 'a>(adapter: Option<fn(&AppCfgInfo) -> Arc<T>>) -> CfgSrc<'a, T> {
    let src: Box<dyn 'a + Fn() -> Arc<T> + Send + Sync> = if let Some(adapter) = adapter {
        Box::new(move || adapter(getAppConfiguration().as_ref()))
    } else {
        Box::new(nilCfgSrc)
    };

    CfgSrc { src }
}

pub fn makeCfgSrc<'a, T: 'a>(adapter: Option<fn(&AppCfgInfo) -> Arc<T>>) -> CfgSrc<'a, T> {
    if let Some(adapter) = adapter {
        CfgSrc::new(move || adapter(getAppConfiguration().as_ref()))
    } else {
        CfgSrc::new(nilCfgSrc)
    }
}
