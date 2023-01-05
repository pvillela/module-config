use pullwithpushoverride::{
    config::cfg_src::update_cfg_src,
    fs::baz::{baz, bazCfgSrc, BazCfgInfo},
};

fn main() {
    update_cfg_src(&bazCfgSrc, || BazCfgInfo {
        w: "baz".to_owned(),
    });

    baz();
}
