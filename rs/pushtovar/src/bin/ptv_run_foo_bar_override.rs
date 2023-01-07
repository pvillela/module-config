use pushtovar::{
    fs::bar_bf::{BarBfCfgInfo, BAR_BF_CFG_SRC},
    fs::foo_sfl::{foo_sfl, FooSflCfgInfo, FOO_SFL_CFG_SRC},
    fwk::cfg_src::update_cfg_src_with_fn,
};

fn main() {
    update_cfg_src_with_fn(&FOO_SFL_CFG_SRC, || FooSflCfgInfo {
        x: "foo".to_owned(),
    });

    update_cfg_src_with_fn(&BAR_BF_CFG_SRC, || BarBfCfgInfo { z: 99 });

    foo_sfl();
}
