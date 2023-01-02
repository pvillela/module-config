use crate::config::app_cfg_info::{getAppConfiguration, AppCfgInfo};
use std::{cell::RefCell, sync::Arc};

fn nilCfgSrc<T>() -> Arc<T> {
    panic!("Module used before being initialized");
}

pub struct CfgSrc<'a, T: 'a> {
    src: Box<dyn 'a + Fn() -> Arc<T>>,
}

impl<'a, T: 'a> CfgSrc<'a, T> {
    pub fn setup(&mut self, src: Box<dyn Fn() -> Arc<T>>) {
        self.src = src;
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

    // if adapter.is_some() {
    //     let adapter = adapter.unwrap();
    //     let x = adapter(getAppConfiguration().as_ref());
    //     Box::new(move || x.clone())
    // } else {
    //     Box::new(nilCfgSrc)
    // }
}

// Change this to be the `new` static method in CfgSrc
pub fn makeCfgSrc<'a, T: 'a>(adapter: Option<fn(&AppCfgInfo) -> Arc<T>>) -> CfgSrc<'a, T> {
    if let Some(adapter) = adapter {
        // let x = adapter(getAppConfiguration().as_ref());
        CfgSrc {
            src: Box::new(move || adapter(getAppConfiguration().as_ref())),
        }
    } else {
        CfgSrc {
            src: Box::new(nilCfgSrc),
        }
    }

    // if adapter.is_some() {
    //     let adapter = adapter.unwrap();
    //     let x = adapter(getAppConfiguration().as_ref());
    //     Box::new(move || x.clone())
    // } else {
    //     Box::new(nilCfgSrc)
    // }
}
