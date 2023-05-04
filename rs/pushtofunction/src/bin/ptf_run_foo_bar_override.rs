use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{RefreshMode, Src};
use pushtofunction::fs::{foo_sfl_c, FooSflCfgDeps, FooSflDeps};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn bar_ovd_bf() -> String {
    let cfg = BarBfCfgInfo {
        u: 33,
        v: "bar_override".to_owned(),
    };
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_ovd_bf";
    bar_core(u, v)
}

/// The creation of FooSflCfgDeps needs to happen in the same thread as the invocation
/// of foo_sfl_c, thus it has been encapsulated in a function that can be called from
/// two threads below.
fn foo_cfg_deps() -> FooSflCfgDeps {
    let foo_deps = FooSflDeps {
        bar_bf: Rc::new(bar_ovd_bf),
    };

    FooSflCfgDeps::new(
        Src::new_boxed(|| FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        }),
        RefreshMode::Refreshable(Duration::from_millis(0)),
        foo_deps,
    )
}

fn main() {
    {
        println!("Running foo_sfl with config-deps override in 2 threads.");

        let handle = thread::spawn(move || foo_sfl_c(foo_cfg_deps())());

        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || foo_sfl_c(foo_cfg_deps())());

        let res = handle.join().unwrap();
        println!("{}", res);
    }
}
