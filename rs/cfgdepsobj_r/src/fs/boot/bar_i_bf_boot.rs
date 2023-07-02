use crate::fs::BarIBfS;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarIBfCfgInfo;
use std::sync::{Arc, OnceLock};

fn bar_i_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarIBfCfgInfo {
    BarIBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn get_bar_i_bf_s_with_app_cfg(app_cfg_src: fn() -> Arc<AppCfgInfo>) -> BarIBfS {
    BarIBfS {
        cfg: bar_i_bf_cfg_adapter(&app_cfg_src()),
    }
}

pub fn get_bar_i_bf_s() -> &'static BarIBfS {
    static BAR_I_BF_S: OnceLock<BarIBfS> = OnceLock::new();
    BAR_I_BF_S.get_or_init(|| get_bar_i_bf_s_with_app_cfg(get_app_configuration))
}
