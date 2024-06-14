use common::config::AppCfgInfo;
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDepsS, CfgRefCellRc, RefreshMode};

pub type BarBfCfg = CfgArcSwapArc<BarBfCfgInfo>;

pub type BarBfT = fn() -> String;

fn bar_bf() -> String {
    // This is to demonstrate use of global config instead of thread-local.
    let _cfg = BAR_BF_CFG.get_cfg().get_cfg();

    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

static BAR_BF_CFG: CfgDepsS<BarBfCfg, ()> = CfgDepsS::new();

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = cfg_to_thread_local(BAR_BF_CFG.get_cfg());
}

pub fn get_bar_bf_raw(cfg: BarBfCfg) -> BarBfT {
    let _ = BAR_BF_CFG.set_cfg_lenient(cfg);
    bar_bf
}

fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_bf_with_app_cfg(
    app_cfg_src: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> BarBfT {
    get_bar_bf_raw(BarBfCfg::new_boxed_with_cfg_adapter(
        app_cfg_src,
        bar_bf_cfg_adapter,
        refresh_mode,
    ))
}
