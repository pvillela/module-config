use super::bar_bf;
use crate::{
    config::{get_app_configuration, AppCfgInfo},
    fwk::{CfgDeps, RefreshMode},
};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let (cfg, FooSflDeps { bar_bf }) = CfgDeps::get(&FOO_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
}

pub static FOO_SFL_CFG_DEPS: Lazy<ArcSwap<CfgDeps<FooSflCfgInfo, FooSflDeps>>> =
    Lazy::new(move || {
        ArcSwap::new(CfgDeps::new_with_cfg_adapter(
            get_app_configuration,
            foo_sfl_cfg_adapter,
            RefreshMode::NoRefresh,
            FooSflDeps { bar_bf },
        ))
    });

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
