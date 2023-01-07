use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub z: i32,
}

pub struct BarBfCfgSrc {
    pub get: Box<dyn Fn() -> Arc<BarBfCfgInfo>>,
}

pub type BarBfT = Box<dyn Fn()>;

pub fn bar_bf_c(cfg: BarBfCfgSrc) -> BarBfT {
    Box::new(move || println!("{}", (cfg.get)().z))
}
