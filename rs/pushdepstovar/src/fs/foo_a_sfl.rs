use crate::fwk::CfgDeps;
use core::pin::Pin;
use once_cell::sync::OnceCell;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooASflDeps {
    pub bar_a_bf: fn() -> Pin<Box<dyn Future<Output = String> + Send + Sync>>,
}

pub static FOO_A_SFL_CFG_DEPS: OnceCell<CfgDeps<FooASflCfgInfo, FooASflDeps>> = OnceCell::new();

pub async fn foo_a_sfl() -> String {
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf().await)
}
