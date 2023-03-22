use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{arc_pin_async_fn_web, CfgDepsRefCellId, RcPinFnWeb};
use std::time::Duration;
use tokio::time::sleep;

pub type BarAwBfT = RcPinFnWeb<u64, String>;

pub type BarAwBfCfgDeps = CfgDepsRefCellId<BarABfCfgInfo, ()>;

pub fn bar_aw_bf_c(cfg_deps: BarAwBfCfgDeps) -> BarAwBfT {
    let f = move |sleep_millis: u64| {
        let cfg = cfg_deps.get_cfg();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    arc_pin_async_fn_web(f)
}
