use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgArcSwapArc};
use std::time::Duration;
use tokio::time::sleep;

pub type BarABfT = ArcPinFn<u64, String>;

pub type BarABfCfg = CfgArcSwapArc<BarABfCfgInfo>;

pub fn bar_a_bf_c(cfg: BarABfCfg) -> BarABfT {
    let f = move |sleep_millis: u64| {
        let cfg = cfg.get_cfg();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    arc_pin_async_fn(f)
}
