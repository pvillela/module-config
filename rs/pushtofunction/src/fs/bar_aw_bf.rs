use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{arc_pin_async_fn_web, ArcPinFnWeb, CfgDepsRefCellId};
use std::time::Duration;
use tokio::time::sleep;

pub type BarAwBfT = ArcPinFnWeb<u64, String>;

pub type BarAwBfCfgDeps = CfgDepsRefCellId<BarABfCfgInfo, ()>;

pub fn bar_aw_bf_c(cfg_deps: BarAwBfCfgDeps) -> BarAwBfT {
    let f = move |sleep_millis: u64| {
        let (cfg, _) = cfg_deps.get_cfg_deps();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    arc_pin_async_fn_web(f)
}
