mod common;

use common::common_test;
use pushdepstovar::fs::{BarABfCfgInfo, FooASflCfgInfo};
use tokio;

#[tokio::test]
async fn test1() {
    let res = common_test(
        FooASflCfgInfo {
            a: "foo_a_test1".to_owned(),
            b: 1,
        },
        BarABfCfgInfo {
            u: 11,
            v: "bar_a_test1".to_owned(),
        },
    )
    .await;
    assert_eq!(
        res,
        Some("fooSfl(): a=foo_a_test1-foo, b=4, bar=(barBf(): u=12, v=bar_a_test1-bar)".to_owned())
    );
}
