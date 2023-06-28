use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_to_thread_local, CfgArcSwapArc, CfgDeps, CfgRefCellRc, RefreshMode};

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

static BAR_BF_CFG: CfgDeps<BarBfCfg, ()> = CfgDeps::init_with_cfg(|| {
    BarBfCfg::new_boxed_with_cfg_adapter(
        get_app_configuration, // use `|| todo!()` before get_app_configuration exists
        bar_bf_cfg_adapter,    // use `|_| todo!()` before bar_bf_cfg_adapter exists
        RefreshMode::NoRefresh,
    )
});

thread_local! {
    pub static BAR_BF_CFG_TL: CfgRefCellRc<BarBfCfgInfo> = cfg_to_thread_local(BAR_BF_CFG.get_cfg());
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn bar_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarBfCfgInfo {
    BarBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}
