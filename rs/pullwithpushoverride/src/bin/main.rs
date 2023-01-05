use pullwithpushoverride::{
    config::{app_cfg_info::refresh_app_configuration, cfg_src::update_cfg_src_with_fn},
    fs::bar_bf::{bar_bf, BarBfCfgInfo, BAR_BF_CFG_SRC},
};

fn main() {
    bar_bf();

    refresh_app_configuration();

    bar_bf();

    // Override BAR_CFG_SRC

    fn another_bar_src() -> BarBfCfgInfo {
        BarBfCfgInfo { z: 99 }
    }

    update_cfg_src_with_fn(&BAR_BF_CFG_SRC, another_bar_src);

    bar_bf();
}
