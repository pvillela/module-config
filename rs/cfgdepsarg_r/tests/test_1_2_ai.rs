mod common_test_ai;

use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common_test_ai::common_test;
use tokio;

mod t1 {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = common_test(
            FooSflCfgInfo {
                a: "foo_ai_test1".to_owned(),
                b: 1,
            },
            BarBfCfgInfo {
                u: 11,
                v: "bar_ai_test1".to_owned(),
            },
        )
        .await;
        assert_eq!(
            res,
            Some("foo: a=foo_ai_test1-foo, b=4, bar=(bar: u=12, v=bar_ai_test1-bar)".to_owned())
        );
    }
}

mod t2 {
    use super::*;

    #[tokio::test]
    async fn test2() {
        let res = common_test(
            FooSflCfgInfo {
                a: "foo_ai_test2".to_owned(),
                b: 2,
            },
            BarBfCfgInfo {
                u: 22,
                v: "bar_ai_test2".to_owned(),
            },
        )
        .await;
        assert_eq!(
            res,
            Some("foo: a=foo_ai_test2-foo, b=5, bar=(bar: u=23, v=bar_ai_test2-bar)".to_owned())
        );
    }
}
