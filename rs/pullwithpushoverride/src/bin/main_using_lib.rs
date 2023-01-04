use pullwithpushoverride::{
    config::{
        app_cfg_info::{init_app_configuration, refresh_app_configuration},
        cfg_src::CfgSrc,
    },
    fs::bar_bf::{barBf, barBfCfgSrc, barBfCfgSrc_arc, barBf_arc, BarBfCfgInfo},
};

use std::sync::Arc;

fn main() {
    init_app_configuration();

    // With Arc

    barBf_arc();

    fn another_bar_src_arc() -> Arc<BarBfCfgInfo> {
        Arc::new(BarBfCfgInfo { z: 99 })
    }

    barBfCfgSrc_arc.store(Arc::new(CfgSrc::new(another_bar_src_arc)));

    barBf_arc();

    // Without Arc

    barBf();

    refresh_app_configuration();

    barBf();

    // Override BAR_CFG_SRC

    fn another_bar_src() -> BarBfCfgInfo {
        BarBfCfgInfo { z: 99 }
    }

    barBfCfgSrc.store(Arc::new(CfgSrc::new(another_bar_src)));

    barBf();
}
