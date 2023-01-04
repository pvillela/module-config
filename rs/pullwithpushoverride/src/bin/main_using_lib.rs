use pullwithpushoverride::{
    config::cfg_src::CfgSrc,
    fs::bar_bf::{barBf, barBfCfgSrc, barBfCfgSrc_arc, barBf_arc, BarBfCfgInfo},
};

use std::sync::Arc;

fn main() {
    // With Arc

    barBf_arc();

    fn another_bar_src_arc() -> Arc<BarBfCfgInfo> {
        Arc::new(BarBfCfgInfo { z: 99 })
    }

    barBfCfgSrc_arc.store(Arc::new(CfgSrc::new(another_bar_src_arc)));

    barBf_arc();

    // Without Arc

    barBf();

    fn another_bar_src() -> BarBfCfgInfo {
        BarBfCfgInfo { z: 99 }
    }

    barBfCfgSrc.store(Arc::new(CfgSrc::new(another_bar_src)));

    barBf();
}
