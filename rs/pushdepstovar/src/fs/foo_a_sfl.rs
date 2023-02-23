use crate::fwk::CfgDeps;
use core::pin::Pin;
use once_cell::sync::OnceCell;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

pub struct FooASflDeps {
    pub bar_a_bf:
        Box<dyn Fn(u64) -> Pin<Box<dyn Future<Output = String> + Send + Sync>> + Send + Sync>,
}

pub static FOO_A_SFL_CFG_DEPS: OnceCell<CfgDeps<FooASflCfgInfo, FooASflDeps>> = OnceCell::new();

pub async fn foo_a_sfl(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf(0).await)
}
