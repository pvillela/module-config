use std::sync::Arc;

use pushtovar::fs::{foo_sfl, BarBfCfgInfo, FooSflCfgInfo, BAR_BF_CFG_SRC, FOO_SFL_CFG_SRC};
use pushtovar::fwk::update_cfg_src_with_fn;

fn main() {
    update_cfg_src_with_fn(&FOO_SFL_CFG_SRC, || {
        Arc::new(FooSflCfgInfo {
            x: "foo".to_owned(),
        })
    });

    update_cfg_src_with_fn(&BAR_BF_CFG_SRC, || Arc::new(BarBfCfgInfo { z: 99 }));

    foo_sfl();
}
