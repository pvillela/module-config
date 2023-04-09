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

#[allow(unused)]
use std::sync::Arc;

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
            // bar_a_bf: Arc::new(|_| todo!()), // do this before bar_a_bf exist
            bar_a_bf: arc_pin_async_fn(bar_a_bf), // replace above with this after bar_a_bf has been created
        },
    )
});

thread_local! {
pub static FOO_A_SFL_CFG: FooASflCfg =
    FooASflCfg::new_boxed_with_cfg_adapter_and_override(
        FOO_A_SFL_CFG_OVERRIDE.get(),
        get_app_configuration, // use `|| todo!()` before get_app_configuration exists
        foo_a_sfl_cfg_adapter, // use `|_| todo!()` before foo_a_sfl_cfg_adapter exists
        RefreshMode::NoRefresh,
    )
}

pub static FOO_A_SFL_CFG_OVERRIDE: OnceCell<FooASflCfgOvd> = OnceCell::new();
pub static FOO_A_SFL_DEPS_OVERRIDE: OnceCell<FooASflDeps> = OnceCell::new();

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
