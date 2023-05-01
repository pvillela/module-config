use crate::fs::BAR_I_BF_CFG;
use common::config::AppCfgInfo;
use common::fs_data::BarIBfCfgInfo;
use common::fwk::init_option;
use std::sync::Arc;

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

pub fn bar_i_bf_init(origin: fn() -> Arc<AppCfgInfo>) {
    unsafe {
        init_option(&mut BAR_I_BF_CFG, bar_i_bf_cfg_adapter(&origin()));
    }
}
