use crate::config::get_app_configuration;
use crate::config::AppCfgInfo;
use crate::fs::boot::foo_sfl_boot;
use crate::fs::FooSflT;
use std::sync::Arc;

pub fn make_foo_sfl() -> FooSflT {
    foo_sfl_boot(get_app_configuration)
}

pub fn make_foo_sfl1() -> FooSflT {
    let app_cfg_src1 = move || {
        Arc::new(AppCfgInfo {
            x: "foo".to_owned(),
            y: 99,
        })
    };
    foo_sfl_boot(app_cfg_src1)
}
