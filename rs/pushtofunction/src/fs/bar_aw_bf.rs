use common::config::AppCfgInfo;
use common::fs_data::{BarABfCfgInfo, BarAwBfCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{rc_pin_async_fn_wss, AppCfg, CfgRefCellId, RcPinFnWss};
use std::time::Duration;
use tokio::time::sleep;

pub type BarAwBfT = RcPinFnWss<u64, String>;

pub type BarAwBfCfg = CfgRefCellId<BarABfCfgInfo>;

pub fn bar_aw_bf_c(cfg: BarAwBfCfg) -> BarAwBfT {
    let f = move |sleep_millis: u64| {
        let cfg = cfg.get_cfg();
        async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            let u = cfg.u;
            let v = cfg.v.clone();
            bar_core(u, v)
        }
    };
    rc_pin_async_fn_wss(f)
}

fn bar_aw_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAwBfCfgInfo {
    BarAwBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_aw_bf_boot(app_cfg: AppCfg<AppCfgInfo>) -> BarAwBfT {
    let bar_aw_bf_cfg = BarAwBfCfg::new_boxed_with_cfg_adapter(
        app_cfg.app_src,
        bar_aw_bf_cfg_adapter,
        app_cfg.refresh_mode,
    );
    bar_aw_bf_c(bar_aw_bf_cfg)
}
