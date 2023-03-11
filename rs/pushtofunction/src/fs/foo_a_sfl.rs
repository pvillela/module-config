use super::bar_a_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    box_pin_async_fn, BoxPinFn, CfgDepsArcSwapArc, CfgDepsDefault, CfgDepsMut, CfgDepsRefCellArc,
    RefreshMode,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

type FooSflCfgInfo = common::fs_data::FooSflCfgInfo;

type FooAIn = common::fs_data::FooAIn;

type FooAOut = common::fs_data::FooAOut;

pub type FooASflT = BoxPinFn<FooAIn, FooAOut>;

type FooASflCfgDeps = CfgDepsDefault<FooSflCfgInfo, FooASflDeps>;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

impl std::fmt::Debug for FooASflDeps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<FooASflDeps>")
    }
}

pub type FooSflT = Box<dyn FnMut() -> String>;

#[derive(Clone, Debug)]
struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_a_sfl_c(cfg_deps: FooASflCfgDeps) -> FooASflT {
    let mut x = cfg_deps.get_inner_clone();
    let f = move |input| async {
        let FooAIn { sleep_millis } = input;
        sleep(Duration::from_millis(sleep_millis)).await;
        let (cfg, deps, _) = x.get_mut();
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_bf = deps.bar_a_bf;
        let bar_ret = bar_bf(0).await;
        let res = foo_core(a, b, bar_ret);
        FooAOut { res }
    };
    box_pin_async_fn(f)
}

// pub async fn foo_a_sfl(input: FooIn) -> FooOut {
//     let FooIn { sleep_millis } = input;
//     sleep(Duration::from_millis(sleep_millis)).await;
//     // Block below used as workaround for case when CfgDepsDefault is based on Rc (instead of Arc)
//     // to make compiler see the Rc is dropped before it leaks into the Future.
//     let (a, b, bar) = {
//         let (cfg, d) = FOO_A_SFL_CFG_DEPS.with(CfgDepsDefault::get);
//         let a = cfg.a.clone();
//         let b = cfg.b;
//         (a, b, d.bar_a_bf)
//     };
//     let bar_res = bar(0).await;
//     let res = foo_core(a, b, bar_res);
//     FooOut { res }
// }

// thread_local! {
// pub static FOO_A_SFL_CFG_DEPS: CfgDepsDefault<FooSflCfgInfo, FooASflDeps> =
//     CfgDepsDefault::new_with_cfg_adapter(
//         get_app_configuration,
//         foo_a_sfl_cfg_adapter,
//         RefreshMode::NoRefresh,
//         // RefreshMode::Refreshable(Duration::from_millis(60)),
//         FooASflDeps {
//             bar_a_bf: box_pin_async_fn(bar_a_bf),
//         },
//     )
// }

// fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
//     FooSflCfgInfo {
//         a: app_cfg.x.clone(),
//         b: app_cfg.y,
//     }
// }
