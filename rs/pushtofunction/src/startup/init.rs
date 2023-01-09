use crate::config::get_app_configuration;
use crate::config::AppCfgInfo;
use crate::fs::boot::foo_sfl_boot;
use std::sync::Arc;

pub fn foo_sfl() {
    let f = foo_sfl_boot(get_app_configuration);
    f()
}

pub fn foo_sfl1() {
    let app_cfg_src1 = move || {
        Arc::new(AppCfgInfo {
            x: "foo".to_owned(),
            y: 99,
        })
    };
    let f = foo_sfl_boot(app_cfg_src1);
    f()
}
