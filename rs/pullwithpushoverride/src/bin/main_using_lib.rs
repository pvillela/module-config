use pullwithpushoverride::{config::app_cfg_info::AppCfgInfo, getAppConfiguration, makeCfgSrc};
use std::{cell::RefCell, sync::Mutex};

static y: Mutex<RefCell<i32>> = Mutex::new(RefCell::new(1));

fn foo() {
    println!("{}", y.lock().as_ref().unwrap().borrow())
}

fn main() {
    let app_cfg = getAppConfiguration();
    println!("AppConfiguration: {:?}", app_cfg);

    foo();

    let cfg_src = makeCfgSrc::<AppCfgInfo>(None);
    // cfg_src.borrow()();
    let _ = cfg_src.replace(Box::new(getAppConfiguration));
    let cfg = cfg_src.borrow()();
    println!("{:?}", cfg);
}
