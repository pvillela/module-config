use crate::fs::get_bar_a_bf_with_app_cfg;
use common::config::AppCfgInfo;
use common::fwk::RefreshMode;
use common::{
    fs_data::{FooAIn, FooAOut, FooASflCfgInfo},
    fs_util::foo_core,
    fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDepsS, CfgRefCellRc, Pinfn},
    pin_async_fn,
};
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub type FooASflT = Pinfn<FooAIn, FooAOut>;

pub struct FooASflDeps {
    pub bar_a_bf: Pinfn<u64, String>,
}

async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    let FooASflDeps { bar_a_bf } = FOO_A_SFL_CFG_DEPS.get_deps();

    // This is to demonstrate use of global config instea of thread-local.
    let _cfg = FOO_A_SFL_CFG_DEPS.get_cfg().get_cfg();

    let (a, b) = {
        let cfg = FOO_A_SFL_CFG_TL.with(|c| c.get_cfg());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar_a_bf(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

pub static FOO_A_SFL_CFG_DEPS: CfgDepsS<FooASflCfg, FooASflDeps> = CfgDepsS::new();

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_to_thread_local(FOO_A_SFL_CFG_DEPS.get_cfg());
}

pub fn get_foo_a_sfl_raw(cfg: FooASflCfg, deps: FooASflDeps) -> FooASflT {
    let _ = FOO_A_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_A_SFL_CFG_DEPS.set_deps_lenient(deps);
    pin_async_fn!(foo_a_sfl)
}

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn get_foo_a_sfl_with_app_cfg(
    app_cfg_src: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> FooASflT {
    // A stereotype should initialize its dependencies.
    let bar_a_bf = get_bar_a_bf_with_app_cfg(app_cfg_src, refresh_mode.clone());
    let deps = FooASflDeps { bar_a_bf };
    get_foo_a_sfl_raw(
        FooASflCfg::new_boxed_with_cfg_adapter(app_cfg_src, foo_a_sfl_cfg_adapter, refresh_mode),
        deps,
    )
}
