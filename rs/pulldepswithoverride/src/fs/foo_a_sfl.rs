use super::bar_a_bf;
use arc_swap::ArcSwap;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{box_pin_async_fn, BoxPinFn, CfgDeps, RefreshMode};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::sleep;

type FooSflCfgInfo = common::fs_data::FooSflCfgInfo;

type FooIn = common::fs_data::FooAIn;

type FooOut = common::fs_data::FooAOut;

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

pub async fn foo_a_sfl(input: FooIn) -> FooOut {
    let FooIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get_from_static(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    let res = format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf(0).await);
    FooOut { res }
}

pub static FOO_A_SFL_CFG_DEPS: Lazy<ArcSwap<CfgDeps<FooSflCfgInfo, FooASflDeps>>> =
    Lazy::new(move || {
        ArcSwap::new(CfgDeps::new_with_cfg_adapter(
            get_app_configuration,
            foo_a_sfl_cfg_adapter,
            RefreshMode::NoRefresh,
            FooASflDeps {
                bar_a_bf: box_pin_async_fn(bar_a_bf),
            },
        ))
    });

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
