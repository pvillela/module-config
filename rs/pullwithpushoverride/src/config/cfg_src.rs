use crate::config::app_cfg_info::{getAppConfiguration, AppCfgInfo};
use std::{cell::RefCell, sync::Arc};

fn nilCfgSrc<T>() -> Arc<T> {
    panic!("Module used before being initialized");
}

pub fn makeCfgSrc<'a, T: 'a>(
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
