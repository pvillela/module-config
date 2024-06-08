use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{CfgRefCellRc, RefreshMode};
use std::rc::Rc;
use std::sync::Arc;

pub type FooSflT = Rc<dyn Fn() -> String>;

pub type FooSflCfg = CfgRefCellRc<FooSflCfgInfo>;

#[derive(Clone)]
pub struct FooSflDeps {
    pub bar_bf: Rc<dyn Fn() -> String>,
}

pub fn foo_sfl_c(cfg: FooSflCfg, deps: FooSflDeps) -> FooSflT {
    let f = move || {
        let cfg = cfg.get_cfg();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = &deps.bar_bf;
        let bar_ret = bar_bf();
        foo_core(a, b, bar_ret)
    };
    Rc::new(f)
}

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
        bar_bf: fs::bar_bf_boot(app_cfg, refresh_mode),
    };
    foo_sfl_c(cfg, deps)
}
