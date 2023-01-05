use pushtovar::{
    config::cfg_src::update_cfg_src_with_fn,
    fs::baz::{baz, BazCfgInfo, BAZ_CFG_SRC},
};

fn main() {
    update_cfg_src_with_fn(&BAZ_CFG_SRC, || BazCfgInfo {
        w: "baz".to_owned(),
    });

    baz();
}
