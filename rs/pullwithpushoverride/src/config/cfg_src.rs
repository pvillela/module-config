use crate::config::app_cfg_info::{getAppConfiguration, AppCfgInfo};

fn nilCfgSrc<T>() -> T {
    panic!("Module used before being initialized");
}

pub struct CfgSrc<T: 'static> {
    src: Box<dyn 'static + Fn() -> T + Send + Sync>,
}

impl<T: 'static> CfgSrc<T> {
    pub fn new(src: impl 'static + Fn() -> T + Send + Sync) -> Self {
        CfgSrc { src: Box::new(src) }
    }

    pub fn from_adapter(adapter: Option<fn(&AppCfgInfo) -> T>) -> CfgSrc<T> {
        makeCfgSrc(adapter)
    }

    pub fn set_src(&mut self, src: impl 'static + Fn() -> T + Send + Sync) {
        self.src = Box::new(src);
    }

    pub fn get(&self) -> T {
        self.src.as_ref()()
    }
}

pub fn makeCfgSrc<T: 'static>(adapter: Option<fn(&AppCfgInfo) -> T>) -> CfgSrc<T> {
    if let Some(adapter) = adapter {
        CfgSrc::new(move || adapter(getAppConfiguration().as_ref()))
    } else {
        CfgSrc::new(nilCfgSrc)
    }
}
