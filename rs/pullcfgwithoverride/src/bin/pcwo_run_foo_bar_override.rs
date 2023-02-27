use pullcfgwithoverride::config::update_cfg_src_with_fn;
use pullcfgwithoverride::fs::{
    foo_sfl, BarBfCfgInfo, FooSflCfgInfo, BAR_BF_CFG_SRC, FOO_SFL_CFG_SRC,
};

fn main() {
    update_cfg_src_with_fn(&FOO_SFL_CFG_SRC, || FooSflCfgInfo {
        x: "foo".to_owned(),
    });

    update_cfg_src_with_fn(&BAR_BF_CFG_SRC, || BarBfCfgInfo { z: 99 });

    foo_sfl();
}
