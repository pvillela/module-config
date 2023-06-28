use std::sync::OnceLock;

/// Represents a combination of configuration and dependencies data structures
/// suitable for use as the type of a static variable.
/// Instantiations may override CfgDepsTpl.
pub struct CfgDeps<C, D> {
    cfg: OnceLock<C>,
    deps: OnceLock<D>,
}

pub trait CfgDepsRaw<C, D> {
    fn raw_cfg(&self) -> &OnceLock<C>;
    fn raw_deps(&self) -> &OnceLock<D>;
}

/// Template for CfgDeps implementations
pub trait CfgDepsTpl<C, D>: CfgDepsRaw<C, D> {
    fn get_cfg(&self) -> &C {
        let cell = self.raw_cfg();
        cell.get_or_init(|| {
            self.get_cfg_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    fn get_deps(&self) -> &D {
        let cell = self.raw_deps();
        cell.get_or_init(|| {
            self.get_deps_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    fn get_cfg_init(&self) -> Option<C> {
        None
    }

    fn get_deps_init(&self) -> Option<D> {
        None
    }

    fn set_cfg_strict(&self, cfg: C) {
        assert!(
            self.raw_cfg().set(cfg).is_ok(),
            "Attempt to set already initialized OnceLock."
        );
    }

    fn set_deps_strict(&self, deps: D) {
        assert!(
            self.raw_deps().set(deps).is_ok(),
            "Attempt to set already initialized OnceLock."
        );
    }

    fn set_cfg_lenient(&self, cfg: C) -> Result<(), C> {
        self.raw_cfg().set(cfg)
    }

    fn set_deps_lenient(&self, deps: D) -> Result<(), D> {
        self.raw_deps().set(deps)
    }
}

impl<C, D> CfgDeps<C, D> {
    pub const fn new() -> Self {
        CfgDeps {
            cfg: OnceLock::new(),
            deps: OnceLock::new(),
        }
    }
}

impl<C, D> CfgDepsRaw<C, D> for CfgDeps<C, D> {
    fn raw_cfg(&self) -> &OnceLock<C> {
        &self.cfg
    }

    fn raw_deps(&self) -> &OnceLock<D> {
        &self.deps
    }
}
