use crate::fwk::{BoxPinFn, CfgDeps};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct FooASflCfgInfo {
    pub a: String,
    pub b: i32,
}

#[derive(Clone)]
pub struct FooASflDeps {
    pub bar_a_bf: BoxPinFn<u64, String>,
}

#[derive(Deserialize)]
pub struct FooAIn {
    pub sleep_millis: u64,
}

#[allow(unused)]
#[derive(Serialize)]
pub struct FooAOut {
    pub res: String,
}

pub static FOO_A_SFL_CFG_DEPS: OnceLock<CfgDeps<FooASflCfgInfo, FooASflDeps>> = OnceLock::new();

pub async fn foo_a_sfl(input: FooAIn) -> FooAOut {
    let FooAIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let (cfg, FooASflDeps { bar_a_bf }) = CfgDeps::get(&FOO_A_SFL_CFG_DEPS);
    let a = cfg.a.clone() + "-foo";
    let b = cfg.b + 3;
    let res = format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_a_bf(0).await);
    FooAOut { res }
}
