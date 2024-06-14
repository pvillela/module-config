use std::time::Duration;
use tokio::time::sleep;

use super::bar_ac_bf;

#[derive(Debug, Clone)]
pub struct FooAcSflCfgInfo {
    pub a: &'static str,
    pub b: i32,
}

type FooAcIn = common::fs_data::FooAIn;

type FooAcOut = common::fs_data::FooAOut;

pub const FOO_AC_SFL_CFG_INFO: FooAcSflCfgInfo = FooAcSflCfgInfo {
    a: "constant_config_info",
    b: 84,
};

pub async fn foo_ac_sfl(input: FooAcIn) -> FooAcOut {
    let FooAcIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let cfg = &FOO_AC_SFL_CFG_INFO;
    let a = cfg.a.to_string() + "-foo";
    let b = cfg.b + 3;
    let res = format!("fooSfl(): a={}, b={}, bar=({})", a, b, bar_ac_bf(0).await);
    FooAcOut { res }
}
