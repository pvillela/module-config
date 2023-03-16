use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgDepsArcSwapArcNc, CfgDepsOvr, RefreshMode};
use once_cell::sync::{Lazy, OnceCell};
use std::time::Duration;
use tokio::time::sleep;

use super::bar_an_bf;

type FooAnSflCfgDeps = CfgDepsArcSwapArcNc<FooASflCfgInfo, FooAnSflDeps>;
type FooAnSflCfgInfo = FooASflCfgInfo;
type FooAnIn = FooAIn;
type FooAnOut = FooAOut;

pub type FooAnSflCfgDepsOvr = CfgDepsOvr<FooASflCfgInfo, FooAnSflDeps>;

#[derive(Clone)]
pub struct FooAnSflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

impl std::fmt::Debug for FooAnSflDeps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<FooAnSflDeps>")
    }
}

pub async fn foo_an_sfl(input: FooAnIn) -> FooAnOut {
    let FooAnIn { sleep_millis } = input;
    let cfg = FOO_AN_SFL_CFG_DEPS.get_cfg();
    let d = FOO_AN_SFL_CFG_DEPS.get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = (d.bar_a_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAnOut { res }
}

pub static FOO_AN_SFL_CFG_DEPS: Lazy<FooAnSflCfgDeps> = Lazy::new(|| {
    FooAnSflCfgDeps::new_with_override(
        FOO_AN_SFL_CFG_DEPS_OVERRIDE.get(),
        get_app_configuration,
        foo_an_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
        FooAnSflDeps {
            bar_a_bf: arc_pin_async_fn(bar_an_bf),
        },
    )
});

pub static FOO_AN_SFL_CFG_DEPS_OVERRIDE: OnceCell<FooAnSflCfgDepsOvr> = OnceCell::new();

fn foo_an_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAnSflCfgInfo {
    FooAnSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
