use super::bar_an_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    arc_pin_async_fn, static_ref_with_override, ArcPinFn, CfgOvd, CfgRefCellRc, RefreshMode,
};
use once_cell::sync::{Lazy, OnceCell};
use std::time::Duration;
use tokio::time::sleep;

type FooAnSflCfg = CfgRefCellRc<FooASflCfgInfo>;
type FooAnSflCfgInfo = FooASflCfgInfo;
type FooAnIn = FooAIn;
type FooAnOut = FooAOut;

pub type FooAnSflCfgOvd = CfgOvd<FooASflCfgInfo>;

pub struct FooAnSflDeps {
    pub bar_an_bf: ArcPinFn<u64, String>,
}

impl std::fmt::Debug for FooAnSflDeps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<FooAnSflDeps>")
    }
}

pub async fn foo_an_sfl(input: FooAnIn) -> FooAnOut {
    let FooAnIn { sleep_millis } = input;
    let FooAnSflDeps { bar_an_bf: bar } = &FOO_AN_SFL_DEPS as &FooAnSflDeps;
    let (a, b) = {
        let cfg = FOO_AN_SFL_CFG.with(|c| c.get_cfg());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar(0).await;
    let res = foo_core(a, b, bar_res);
    FooAnOut { res }
}

pub static FOO_AN_SFL_DEPS: Lazy<&FooAnSflDeps> = Lazy::new(|| {
    static_ref_with_override(
        FOO_AN_SFL_DEPS_OVERRIDE.get(),
        FooAnSflDeps {
            bar_an_bf: arc_pin_async_fn(bar_an_bf),
        },
    )
});

thread_local! {
pub static FOO_AN_SFL_CFG: FooAnSflCfg =
    FooAnSflCfg::new_with_override(
        FOO_AN_SFL_CFG_OVERRIDE.get(),
        get_app_configuration,
        foo_an_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
    )
}

pub static FOO_AN_SFL_CFG_OVERRIDE: OnceCell<FooAnSflCfgOvd> = OnceCell::new();
pub static FOO_AN_SFL_DEPS_OVERRIDE: OnceCell<FooAnSflDeps> = OnceCell::new();

fn foo_an_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAnSflCfgInfo {
    FooAnSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
