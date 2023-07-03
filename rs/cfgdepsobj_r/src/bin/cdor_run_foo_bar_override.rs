use cfgdepsobj_r::fs::{
    foo_sfl_c, BarBfCfg, BarBfD, BarBfS, FooSflCfg, FooSflD, FooSflDeps, FooSflS,
};
use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fs_util::bar_core;
use common::fwk::{RefreshMode, Src};
use std::sync::OnceLock;
use std::thread;

fn bar_ovd_bf_c(s: &BarBfS) -> String {
    let cfg = s.cfg.get_cfg();
    let u = cfg.u * 1000;
    let v = cfg.v.clone() + "-bar_ovd_bf";
    bar_core(u, v)
}

fn main() {
    let bar_bf_cfg = BarBfCfg::new(
        Src::new_boxed(|| BarBfCfgInfo {
            u: 33,
            v: "bar_override".to_owned(),
        }),
        RefreshMode::NoRefresh,
    );
    let bar_bf_s = {
        static BAR_BF_S: OnceLock<BarBfS> = OnceLock::new();
        BAR_BF_S.get_or_init(|| BarBfS { cfg: bar_bf_cfg })
    };
    let bar_bf_d = BarBfD {
        s: &bar_bf_s,
        f: bar_ovd_bf_c,
    };

    let foo_sfl_cfg = FooSflCfg::new(
        Src::new_boxed(|| FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        }),
        RefreshMode::NoRefresh,
    );
    let foo_sfl_deps = FooSflDeps { bar_bf_d };
    let foo_sfl_s = {
        static FOO_SFL_S: OnceLock<FooSflS> = OnceLock::new();
        FOO_SFL_S.get_or_init(|| FooSflS {
            cfg: foo_sfl_cfg,
            deps: foo_sfl_deps,
        })
    };

    let handle = thread::spawn(move || {
        let foo_sfl_d = FooSflD {
            s: &foo_sfl_s,
            f: foo_sfl_c,
        };
        foo_sfl_d.run()
    });
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || {
        let foo_sfl_d = FooSflD {
            s: &foo_sfl_s,
            f: foo_sfl_c,
        };
        foo_sfl_d.run()
    });
    let res = handle.join().unwrap();
    println!("{}", res);
}
