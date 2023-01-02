use pullwithpushoverride::{
    config::app_cfg_info::{getAppConfiguration, AppCfgInfo},
    config::cfg_src::{makeCfgSrc, makeCfgSrc0},
    fs::bar_bf::{barBf, barBfCfgSrc, BarBfCfgInfo},
};

use std::{cell::RefCell, sync::Mutex};
use std::{ops::DerefMut, sync::Arc};

static y: Mutex<RefCell<i32>> = Mutex::new(RefCell::new(1));

fn foo() {
    println!("{}", y.lock().as_ref().unwrap().borrow())
}

fn main0() {
    let app_cfg = getAppConfiguration();
    println!("AppConfiguration: {:?}", app_cfg);

    foo();

    let cfg_src = makeCfgSrc0::<AppCfgInfo>(None);
    // cfg_src.borrow()();
    let _ = cfg_src.replace(Box::new(getAppConfiguration));
    let cfg = cfg_src.borrow()();
    println!("{:?}", cfg);
}

fn main() {
    let app_cfg = getAppConfiguration();
    println!("AppConfiguration: {:?}", app_cfg);

    foo();

    let mut cfg_src = makeCfgSrc::<AppCfgInfo>(None);
    // cfg_src.borrow()();
    cfg_src.set_src(getAppConfiguration);
    let cfg = cfg_src.get();
    println!("{:?}", cfg);

    barBf();

    fn another_bar_src() -> Arc<BarBfCfgInfo> {
        Arc::new(BarBfCfgInfo { z: 99 })
    }

    println!("was getting stuck here");
    let mut x = barBfCfgSrc.lock().unwrap();
    // .expect("Error acquiring mutex for barBfCfgSrc");
    x.set_src(another_bar_src);
    drop(x);

    barBf();
}
