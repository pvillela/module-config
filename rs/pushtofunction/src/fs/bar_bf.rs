use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BarBfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub struct BarBfCfgSrc {
    pub get: Box<dyn Fn() -> Arc<BarBfCfgInfo> + Send + Sync>,
}

pub type BarBfT = Arc<dyn Fn() -> String + Send + Sync>;

pub fn bar_bf_c(cfg: BarBfCfgSrc) -> BarBfT {
    Arc::new(move || {
        let cfg = (cfg.get)();
        let u = cfg.u + 1;
        let v = cfg.v.clone() + "-bar";
        format!("barBf(): u={}, v={}", u, v)
    })
}
