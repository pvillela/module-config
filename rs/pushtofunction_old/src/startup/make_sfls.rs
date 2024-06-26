use crate::fs;
use crate::fs::FooSflT;
use common::config::{get_app_configuration, AppCfgInfo0};
use std::sync::Arc;

pub fn make_foo_sfl() -> FooSflT {
    fs::foo_sfl_boot(get_app_configuration)
}

pub fn make_foo_sfl1() -> FooSflT {
    let app_cfg_src1 = move || {
        Arc::new(AppCfgInfo0 {
            x: "custom".to_owned(),
            y: 84,
            z: true,
        })
    };
    fs::foo_sfl_boot(app_cfg_src1)
}
