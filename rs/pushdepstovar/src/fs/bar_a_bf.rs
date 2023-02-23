use crate::fwk::CfgDeps;
use core::pin::Pin;
use once_cell::sync::OnceCell;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct BarABfCfgInfo {
    pub u: i32,
    pub v: String,
}

pub static BAR_A_BF_CFG_DEPS: OnceCell<CfgDeps<BarABfCfgInfo, ()>> = OnceCell::new();

pub async fn bar_a_bf() -> String {
    let (cfg, _) = CfgDeps::get(&BAR_A_BF_CFG_DEPS);
    let u = cfg.u + 1;
    let v = cfg.v.clone() + "-bar";
    format!("barBf(): u={}, v={}", u, v)
}

pub fn box_nullary_async<T: Send + Sync, F>(
    f: fn() -> F,
) -> Box<dyn Fn() -> Pin<Box<dyn Future<Output = T> + Send + Sync>> + Send + Sync>
where
    F: 'static + Future<Output = T> + Send + Sync,
{
    Box::new(move || Box::pin(f()))
}
