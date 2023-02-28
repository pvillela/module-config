use super::bar_a_bf;
use crate::{
    config::{get_app_configuration, AppCfgInfo},
    fwk::{box_pin_async_fn, BoxPinFn, CfgDeps, RefreshMode},
};
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

#[derive(Deserialize)]
pub struct FooAIn {
    pub sleep_millis: u64,
}

#[allow(unused)]
#[derive(Serialize)]
pub struct FooAOut {
    pub res: String,
}

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    let res = format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf(0).await);
    FooAOut { res }
}

pub static FOO_A_SFL_CFG_DEPS: Lazy<ArcSwap<CfgDeps<FooASflCfgInfo, FooASflDeps>>> =
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

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
