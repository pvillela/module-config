use crate::fs::bar_bf::BarBfT;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FooSflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooSflCfgSrc {
    pub get: Box<dyn Fn() -> Arc<FooSflCfgInfo> + Send + Sync>,
    pub bar: BarBfT,
}

pub type FooSflT = Arc<dyn Fn() -> String + Send + Sync>;

pub fn foo_sfl_c(cfg: FooSflCfgSrc) -> FooSflT {
    let bar_bf = cfg.bar;
    Arc::new(move || {
        let cfg = (cfg.get)();
        let a = cfg.a.clone() + "-foo";
        let b = cfg.b + 3;
        format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_bf())
    })
}
