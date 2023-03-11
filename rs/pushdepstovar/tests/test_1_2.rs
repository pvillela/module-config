mod common;

use crate::common::common_test;
use pushdepstovar::fs::{BarABfCfgInfo, FooASflCfgInfo};
use tokio;

mod t1 {
    use super::*;

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
            Some(
                "fooSfl(): a=foo_a_test1-foo, b=4, bar=(barBf(): u=12, v=bar_a_test1-bar)"
                    .to_owned()
            )
        );
    }
}

mod t2 {
    use super::*;

    #[tokio::test]
    async fn test2() {
        let res = common_test(
            FooASflCfgInfo {
                a: "foo_a_test2".to_owned(),
                b: 2,
            },
            BarABfCfgInfo {
                u: 22,
                v: "bar_a_test2".to_owned(),
            },
        )
        .await;
        assert_eq!(
            res,
            Some(
                "fooSfl(): a=foo_a_test2-foo, b=5, bar=(barBf(): u=23, v=bar_a_test2-bar)"
                    .to_owned()
            )
        );
    }
}
