use super::bar_bf_boot;
use crate::fs::{foo_sfl_c, FooSflCfg, FooSflDeps, FooSflS, FooSflT};
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fwk::RefreshMode;
use std::rc::Rc;
use std::sync::Arc;

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>, refresh_mode: RefreshMode) -> FooSflT {
    let cfg =
        FooSflCfg::new_boxed_with_cfg_adapter(app_cfg, foo_sfl_cfg_adapter, refresh_mode.clone());
    let deps = FooSflDeps {
        bar_bf: bar_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_sfl_s = Rc::new(FooSflS { cfg, deps });
    let f = move || foo_sfl_c(&foo_sfl_s.clone());
    Box::new(f)
}
