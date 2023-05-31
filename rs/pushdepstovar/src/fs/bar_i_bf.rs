use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::get_from_once_cell;
use once_cell::sync::OnceCell;

pub fn bar_i_bf() -> String {
    let cfg = get_from_once_cell(&BAR_I_BF_CFG);
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}

pub static BAR_I_BF_CFG: OnceCell<BarIBfCfgInfo> = OnceCell::new();
