use super::bar_a_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDeps, CfgRefCellRc, Pinfn, RefreshMode};
use common::pin_async_fn;
use std::time::Duration;
use tokio::time::sleep;

pub type FooASflCfg = CfgArcSwapArc<FooASflCfgInfo>;

pub struct FooASflDeps {
    pub bar_a_bf: Pinfn<u64, String>,
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
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

pub static FOO_A_SFL_CFG_DEPS: CfgDeps<FooASflCfg, FooASflDeps> = CfgDeps::init(
    || {
        FooASflCfg::new_boxed_with_cfg_adapter(
            get_app_configuration, // use `|| todo!()` before get_app_configuration exists
            foo_a_sfl_cfg_adapter, // use `|_| todo!()` before foo_a_sfl_cfg_adapter exists
            RefreshMode::NoRefresh,
        )
    },
    || FooASflDeps {
        // bar_a_bf: || todo!(), // do this before bar_a_bf exists
        bar_a_bf: pin_async_fn!(bar_a_bf), // replace above with this after bar_a_bf has been created
    },
);

thread_local! {
    pub static FOO_A_SFL_CFG_TL: CfgRefCellRc<FooASflCfgInfo> = cfg_to_thread_local(FOO_A_SFL_CFG_DEPS.get_cfg());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
