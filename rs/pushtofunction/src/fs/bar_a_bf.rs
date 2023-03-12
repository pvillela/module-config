use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{box_pin_async_fn, BoxPinFn, CfgDepsArcSwapArc};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type BarASflT = BoxPinFn<u64, String>;

pub type BarASflCfgDeps = Arc<CfgDepsArcSwapArc<BarABfCfgInfo, ()>>;

#[derive(Clone)]
pub struct BarASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

#[derive(Clone, Debug)]
pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn bar_a_bf_c(cfg_deps: BarASflCfgDeps) -> BarASflT {
    let f = move |sleep_millis: u64| {
        let cfg_deps = cfg_deps.clone();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let (cfg, _) = cfg_deps.get();
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    box_pin_async_fn(f)
}
