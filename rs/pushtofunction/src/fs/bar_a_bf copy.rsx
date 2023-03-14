use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{rc_pin_async_fn, CfgDepsArcSwapArc, RcPinFn};
use std::time::Duration;
use tokio::time::sleep;

pub type BarASflT = RcPinFn<u64, String>;

pub type BarABfCfgDeps = CfgDepsArcSwapArc<BarABfCfgInfo, ()>;

pub fn bar_a_bf_c(cfg_deps: BarABfCfgDeps) -> BarASflT {
    let f = move |sleep_millis: u64| {
        let (cfg, _) = cfg_deps.get_cfg_deps();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    rc_pin_async_fn(f)
}
