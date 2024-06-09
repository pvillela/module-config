use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{AppCfg, CfgRefCellRc};
use std::rc::Rc;

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

pub fn foo_sfl_boot(app_cfg: AppCfg<AppCfgInfo>) -> FooSflT {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooSflCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        foo_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );
    let deps = FooSflDeps {
        bar_bf: fs::bar_bf_boot(app_cfg),
    };
    foo_sfl_c(cfg, deps)
}
