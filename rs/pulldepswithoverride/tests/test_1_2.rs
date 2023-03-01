mod common_test;

use common::fs_data::BarBfCfgInfo;
use common::fs_data::FooSflCfgInfo;
use common_test::common_test;
use tokio;

mod t1 {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = common_test(
            FooSflCfgInfo {
                a: "foo_a_test1".to_owned(),
                b: 1,
            },
            BarBfCfgInfo {
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
            FooSflCfgInfo {
                a: "foo_a_test2".to_owned(),
                b: 2,
            },
            BarBfCfgInfo {
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
