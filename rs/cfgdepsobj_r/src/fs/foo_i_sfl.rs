use super::BarIBfS;
use common::{fs_data::FooISflCfgInfo, fs_util::foo_core};

pub struct FooISflDeps {
    pub bar_i_bf_s: &'static BarIBfS,
}

pub struct FooISflS {
    pub cfg: FooISflCfgInfo,
    pub deps: FooISflDeps,
}

impl FooISflS {
    pub fn run(&self) -> String {
        let FooISflDeps { bar_i_bf_s } = self.deps;
        let cfg = &self.cfg;
        let a = cfg.a.clone();
        let b = cfg.b;
        let bar_res = bar_i_bf_s.run();
        foo_core(a, b, bar_res)
    }
}
