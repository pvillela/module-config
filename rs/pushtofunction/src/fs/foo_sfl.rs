use crate::fs::bar_bf::BarBfT;
use std::sync::Arc;

#[derive(Debug)]
pub struct FooSflCfgInfo {
    pub x: String,
}

pub struct FooSflCfgSrc {
    pub get: Box<dyn Fn() -> Arc<FooSflCfgInfo>>,
    pub bar: BarBfT,
}

pub type FooSflT = Box<dyn Fn()>;

pub fn foo_sfl_c(cfg: FooSflCfgSrc) -> FooSflT {
    let bar_bff = cfg.bar;
    Box::new(move || {
        println!("{}", (cfg.get)().x);
        bar_bff();
    })
}
