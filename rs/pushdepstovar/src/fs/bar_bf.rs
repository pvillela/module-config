use common::fs_data::BarBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{CfgDef, CfgRefCellRc};
use once_cell::sync::OnceCell;

pub type BarBfCfg = CfgRefCellRc<BarBfCfgInfo>;

pub fn bar_bf() -> String {
    let cfg = BAR_BF_CFG_TL.with(|c| c.get_cfg());
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

thread_local! {
pub static BAR_BF_CFG_TL: BarBfCfg =
    BarBfCfg::new_from_once_cell_def(
        &BAR_BF_CFG_DEF,
    )
}

pub static BAR_BF_CFG_DEF: OnceCell<CfgDef<BarBfCfgInfo>> = OnceCell::new();
