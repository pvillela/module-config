use crate::fs::{get_bar_i_bf_raw, BarIBfT};
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn get_bar_i_bf_with_app_cfg(app_cfg: &AppCfgInfo) -> BarIBfT {
    get_bar_i_bf_raw(bar_i_bf_cfg_adapter(app_cfg))
}
