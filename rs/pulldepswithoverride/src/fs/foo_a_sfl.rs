use super::bar_a_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    arc_pin_async_fn, static_ref_with_override, ArcPinFn, CfgOvd, CfgRefCellRc, RefreshMode,
};
use once_cell::sync::{Lazy, OnceCell};
use std::time::Duration;
use tokio::time::sleep;

type FooASflCfg = CfgRefCellRc<FooASflCfgInfo>;

pub type FooASflCfgOvd = CfgOvd<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf: bar } = &FOO_A_SFL_DEPS as &FooASflDeps;
    let (a, b) = {
        let cfg = FOO_A_SFL_CFG.with(|c| c.get_cfg());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

pub static FOO_A_SFL_DEPS: Lazy<&FooASflDeps> = Lazy::new(|| {
    static_ref_with_override(
        FOO_A_SFL_DEPS_OVERRIDE.get(),
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    )
});

thread_local! {
pub static FOO_A_SFL_CFG: FooASflCfg =
    FooASflCfg::new_with_override(
        FOO_A_SFL_CFG_OVERRIDE.get(),
        get_app_configuration,
        foo_a_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
    )
}

pub static FOO_A_SFL_CFG_OVERRIDE: OnceCell<FooASflCfgOvd> = OnceCell::new();
pub static FOO_A_SFL_DEPS_OVERRIDE: OnceCell<FooASflDeps> = OnceCell::new();

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
