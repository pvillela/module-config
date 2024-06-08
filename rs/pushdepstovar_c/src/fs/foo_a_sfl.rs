use crate::fs::{bar_a_bf, bar_a_bf_init_no_refresh, bar_a_bf_init_refreshable};
use crate::fwk::{box_pin_async_fn, BoxPinFn, CfgDeps, RefreshMode};
use common::config::AppCfgInfo;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

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

pub static FOO_A_SFL_CFG_DEPS: OnceLock<CfgDeps<FooASflCfgInfo, FooASflDeps>> = OnceLock::new();

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    let res = format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf(0).await);
    FooAOut { res }
}

fn foo_a_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooASflCfgInfo {
    FooASflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

fn foo_a_sfl_adapt_cfg_src(
    origin: impl Fn() -> Arc<AppCfgInfo> + 'static + Send + Sync,
    refresh_mode: RefreshMode,
    deps: FooASflDeps,
) {
    CfgDeps::set_with_cfg_adapter(
        &FOO_A_SFL_CFG_DEPS,
        origin,
        foo_a_sfl_cfg_adapter,
        refresh_mode,
        deps,
    );
}

pub fn foo_a_sfl_init_refreshable(app_cfg_src: fn() -> Arc<AppCfgInfo>, cache_ttl: Duration) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_refreshable(app_cfg_src, cache_ttl);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::Refreshable(cache_ttl),
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}

pub fn foo_a_sfl_init_no_refresh(app_cfg_src: fn() -> Arc<AppCfgInfo>) {
    // A stereotype should initialize its dependencies.
    bar_a_bf_init_no_refresh(app_cfg_src);
    foo_a_sfl_adapt_cfg_src(
        app_cfg_src,
        RefreshMode::NoRefresh,
        FooASflDeps {
            bar_a_bf: box_pin_async_fn(bar_a_bf),
        },
    );
}
