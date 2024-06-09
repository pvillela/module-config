use common::config::AppCfgInfo;
use common::fs_data::BarABfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{arc_pin_async_fn, AppCfg, ArcPinFn, CfgArcSwapArc};
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

fn bar_a_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarABfCfgInfo {
    BarABfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_a_bf_boot(app_cfg: AppCfg<AppCfgInfo>) -> BarABfT {
    let bar_a_bf_cfg = BarABfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_a_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    bar_a_bf_c(bar_a_bf_cfg)
}
