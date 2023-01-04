use pullwithpushoverride::{
    config::cfg_src::CfgSrc,
    fs::bar_bf::{barBf, barBfCfgSrc, BarBfCfgInfo},
};

use std::sync::Arc;

fn main() {
    barBf();

    fn another_bar_src() -> Arc<BarBfCfgInfo> {
        Arc::new(BarBfCfgInfo { z: 99 })
    }

    barBfCfgSrc.store(Arc::new(CfgSrc::new(another_bar_src)));

    barBf();
}
