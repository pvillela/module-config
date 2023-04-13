use super::bar_a_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    arc_pin_async_fn, cfg_lazy_to_thread_local, static_ref, ArcPinFn, CfgArcSwapArc, CfgRefCellRc,
    RefreshMode,
};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::sleep;

#[allow(unused)]
use std::sync::Arc;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf: bar } = &FOO_A_SFL_DEPS as &FooASflDeps;
    let (a, b) = {
        let cfg = FOO_A_SFL_CFG_TL.with(|c| c.get_cfg());
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
    static_ref(FooASflDeps {
        // bar_bf: || todo!(), // do this before bar_bf exists
        bar_a_bf: arc_pin_async_fn(bar_a_bf), // replace above with this after bar_bf has been created
    })
});

pub static FOO_A_SFL_CFG: Lazy<FooASflCfg> = Lazy::new(|| {
    FooASflCfg::new_boxed_with_cfg_adapter(
        get_app_configuration, // use `|| todo!()` before get_app_configuration exists
        foo_a_sfl_cfg_adapter, // use `|_| todo!()` before foo_sfl_cfg_adapter exists
        RefreshMode::NoRefresh,
    )
});

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_lazy_to_thread_local(&FOO_A_SFL_CFG);
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
