use pullwithpushoverride::{
    config::{
        app_cfg_info::{init_app_configuration, refresh_app_configuration},
        cfg_src::{update_cfg_src, CfgSrc},
    },
    fs::bar_bf::{barBf, barBfCfgSrc, BarBfCfgInfo},
};

use std::sync::Arc;

fn main() {
    init_app_configuration();

    barBf();

    refresh_app_configuration();

    barBf();

    // Override BAR_CFG_SRC

    fn another_bar_src() -> BarBfCfgInfo {
        BarBfCfgInfo { z: 99 }
    }

    update_cfg_src(&barBfCfgSrc, another_bar_src);

    barBf();
}
