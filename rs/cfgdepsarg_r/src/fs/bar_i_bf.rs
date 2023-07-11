use common::fs_data::BarIBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::CfgDeps;

// Send + Sync below is only to support foo_i_sfl_boot_r.
pub type BarIBfT = dyn Fn(()) -> String + Send + Sync;

pub type BarIBfS = CfgDeps<BarIBfCfgInfo, ()>;

pub fn bar_i_bf_c(s: &BarIBfS, _: ()) -> String {
    let cfg = &s.cfg;
    let u = cfg.u;
    let v = cfg.v.clone();
    bar_core(u, v)
}
