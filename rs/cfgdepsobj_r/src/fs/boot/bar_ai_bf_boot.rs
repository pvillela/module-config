use crate::fs::BarAiBfS;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::BarAiBfCfgInfo;
use std::sync::{Arc, OnceLock};

fn bar_ai_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAiBfCfgInfo {
    BarAiBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

fn get_bar_ai_bf_s_with_app_cfg(app_cfg_src: fn() -> Arc<AppCfgInfo>) -> BarAiBfS {
    BarAiBfS {
        cfg: bar_ai_bf_cfg_adapter(&app_cfg_src()),
    }
}

pub fn get_bar_ai_bf_s() -> &'static BarAiBfS {
    static BAR_AI_BF_S: OnceLock<BarAiBfS> = OnceLock::new();
    BAR_AI_BF_S.get_or_init(|| get_bar_ai_bf_s_with_app_cfg(get_app_configuration))
}
