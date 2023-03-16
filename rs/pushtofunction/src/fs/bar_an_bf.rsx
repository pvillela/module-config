use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgDepsArcSwapArcNc};
use std::time::Duration;
use tokio::time::sleep;

pub type BarAnBfT = ArcPinFn<u64, String>;
type BarAnBfCfgInfo = BarABfCfgInfo;

pub type BarAnBfCfgDeps = CfgDepsArcSwapArcNc<BarAnBfCfgInfo, ()>;

pub fn bar_an_bf_c(cfg_deps: BarAnBfCfgDeps) -> BarAnBfT {
    let f = move |sleep_millis: u64| {
        let cfg = cfg_deps.get_cfg();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    arc_pin_async_fn(f)
}
