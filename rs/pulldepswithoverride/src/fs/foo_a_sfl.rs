use super::bar_a_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::{FooAIn, FooAOut, FooASflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgDepsDefault, CfgDepsOvr, RefreshMode};
use once_cell::sync::OnceCell;
use std::time::Duration;
use tokio::time::sleep;

type FooASflCfgDeps = CfgDepsDefault<FooASflCfgInfo, FooASflDeps>;

pub type FooASflCfgDepsOvr = CfgDepsOvr<FooASflCfgInfo, FooASflDeps>;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: ArcPinFn<u64, String>,
}

impl std::fmt::Debug for FooASflDeps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<FooASflDeps>")
    }
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    // Block below used as workaround for case when CfgDepsDefault is based on Rc (instead of Arc)
    // to make compiler see the Rc is dropped before it leaks into the Future.
    let (a, b, bar) = {
        let (cfg, d) = FOO_A_SFL_CFG_DEPS.with(|c| c.get_cfg_deps());
        let a = cfg.a.clone();
        let b = cfg.b;
        (a, b, d.bar_a_bf)
    };
    sleep(Duration::from_millis(sleep_millis)).await;
    let bar_res = bar(0).await;
    let res = foo_core(a, b, bar_res);
    FooAOut { res }
}

thread_local! {
pub static FOO_A_SFL_CFG_DEPS: FooASflCfgDeps =
    FooASflCfgDeps::new_with_override(
        FOO_A_SFL_CFG_DEPS_OVERRIDE.get(),
        get_app_configuration,
        foo_a_sfl_cfg_adapter,
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: arc_pin_async_fn(bar_a_bf),
        },
    )
}

pub static FOO_A_SFL_CFG_DEPS_OVERRIDE: OnceCell<FooASflCfgDepsOvr> = OnceCell::new();

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
