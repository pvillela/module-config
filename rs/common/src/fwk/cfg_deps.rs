use std::sync::OnceLock;

/// Represents a combination of configuration and dependencies data structures
/// suitable for use as the type of a static variable.
pub struct CfgDeps<C, D> {
    cfg: OnceLock<C>,
    deps: OnceLock<D>,
    cfg_init: Option<fn() -> C>,
    deps_init: Option<fn() -> D>,
}

impl<C, D> CfgDeps<C, D> {
    pub const fn new() -> Self {
        CfgDeps {
            cfg: OnceLock::new(),
            deps: OnceLock::new(),
            cfg_init: None,
            deps_init: None,
        }
    }

    pub const fn init(cfg_init: fn() -> C, deps_init: fn() -> D) -> Self {
        CfgDeps {
            cfg: OnceLock::new(),
            deps: OnceLock::new(),
            cfg_init: Some(cfg_init),
            deps_init: Some(deps_init),
        }
    }

    pub fn get_cfg(&self) -> &C {
        self.cfg.get_or_init(|| {
            self.get_cfg_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    pub fn get_deps(&self) -> &D {
        self.deps.get_or_init(|| {
            self.get_deps_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    pub fn set_cfg_strict(&self, cfg: C) {
        assert!(
            self.cfg.set(cfg).is_ok(),
            "Attempt to set already initialized OnceLock."
        );
    }

    pub fn set_deps_strict(&self, deps: D) {
        assert!(
            self.deps.set(deps).is_ok(),
            "Attempt to set already initialized OnceLock."
        );
    }

    pub fn set_cfg_lenient(&self, cfg: C) -> Result<(), C> {
        self.cfg.set(cfg)
    }

    pub fn set_deps_lenient(&self, deps: D) -> Result<(), D> {
        self.deps.set(deps)
    }

    fn get_cfg_init(&self) -> Option<C> {
        match self.cfg_init {
            Some(f) => Some(f()),
            None => None,
        }
    }

    fn get_deps_init(&self) -> Option<D> {
        match self.deps_init {
            Some(f) => Some(f()),
            None => None,
        }
    }
}
