use crate::fs;
use crate::fs::bar_bf::BarBfT;
use crate::fwk::const_or_adapt_by_ref;
use common::config::AppCfgInfo;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooSflCfgSrc {
    pub get: Box<dyn Fn() -> Arc<FooSflCfgInfo> + Send + Sync>,
    pub bar: BarBfT,
}

pub type FooSflT = Arc<dyn Fn() -> String + Send + Sync>;

pub fn foo_sfl_c(cfg: FooSflCfgSrc) -> FooSflT {
    let bar_bf = cfg.bar;
    Arc::new(move || {
        let cfg = (cfg.get)();
        let a = cfg.a.clone() + "-foo";
        let b = cfg.b + 3;
        format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
    })
}

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub static FOO_SFL_CFG_INFO_OVERRIDE: OnceLock<FooSflCfgInfo> = OnceLock::new();

pub fn foo_sfl_boot(app_cfg: fn() -> AppCfgInfo) -> FooSflT {
    let get = const_or_adapt_by_ref(
        FOO_SFL_CFG_INFO_OVERRIDE.get(),
        app_cfg,
        foo_sfl_cfg_adapter,
    );
    let foo_sfl_cfg_src = FooSflCfgSrc {
        get,
        bar: fs::bar_bf_boot(app_cfg),
    };
    foo_sfl_c(foo_sfl_cfg_src)
}
