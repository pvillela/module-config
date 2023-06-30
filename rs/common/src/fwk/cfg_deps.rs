use std::sync::OnceLock;

/// Represents a combination of configuration and dependencies data structures for function
/// stereotypes, suitable for use as the type of a static variable.
///
/// This data structure encapsulates the following elements, where `C` is the type of configurations
/// and `D` is the type of dependencies:
/// - `cfg: OnceLock<C>`
/// - `deps: OnceLock<D>`
/// - `cfg_init: Option<fn() -> C>`
/// - `deps_init: Option<fn() -> D>`
///
/// When created, `cfg` and `deps` are empty, while `cfg_init` and `deps_init` may or may not be
/// empty.
///
/// If `cfg_init` and/or `deps_init` is/are not empty, they will be used to lazily initialize `cfg`
/// and/or deps` when `cfg` and/or `deps` are first read, unless the values of `cfg` and/or `deps`
/// have been previously set by other means (see [CfgDeps] methods).
pub struct CfgDeps<C, D> {
    cfg: OnceLock<C>,
    deps: OnceLock<D>,
    cfg_init: Option<fn() -> C>,
    deps_init: Option<fn() -> D>,
}

impl<C, D> CfgDeps<C, D> {
    /// Creates an uninitialized instance with empty `cfg_init` and `deps_init`.
    pub const fn new() -> Self {
        CfgDeps {
            cfg: OnceLock::new(),
            deps: OnceLock::new(),
            cfg_init: None,
            deps_init: None,
        }
    }

    /// Creates an uninitialized instance with with the specified values of `cfg_init` and `deps_init`.
    pub const fn lazy_init(cfg_init: fn() -> C, deps_init: fn() -> D) -> Self {
        CfgDeps {
            cfg: OnceLock::new(),
            deps: OnceLock::new(),
            cfg_init: Some(cfg_init),
            deps_init: Some(deps_init),
        }
    }

    /// Returns the value of `cfg` if it exists; otherwise, if `cfg_init` is not empty, it uses `cfg_init`
    /// to initialize `cfg` and then returns the value; otherwise, it panics.
    pub fn get_cfg(&self) -> &C {
        self.cfg.get_or_init(|| {
            self.get_cfg_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    /// Returns the value of `deps` if it exists; otherwise, if `deps_init` is not empty, it uses `deps_init`
    /// to initialize `deps` and then returns the value; otherwise, it panics.
    pub fn get_deps(&self) -> &D {
        self.deps.get_or_init(|| {
            self.get_deps_init()
                .expect("Access to uninitialized OnceLock.")
        })
    }

    /// Sets the value of `cfg` if it is empty, panics otherwise.
    pub fn set_cfg_strict(&self, cfg: C) {
        assert!(
            self.cfg.set(cfg).is_ok(),
            "Attempt to set already initialized cfg."
        );
    }

    /// Sets the value of `deps` if it is empty, panics otherwise.
    pub fn set_deps_strict(&self, deps: D) {
        assert!(
            self.deps.set(deps).is_ok(),
            "Attempt to set already initialized deps."
        );
    }

    /// Sets the values of `cfg` and/or `deps` if either is empty, panics otherwise.
    pub fn set_strict(&self, cfg: C, deps: D) {
        self.set_cfg_strict(cfg);
        self.set_deps_strict(deps);
    }

    /// Sets the value of `cfg` if it is empty, returns a result that contains an error if `cfg`
    /// is not empty.
    pub fn set_cfg_lenient(&self, cfg: C) -> Result<(), C> {
        self.cfg.set(cfg)
    }

    /// Sets the value of `deps` if it is empty, returns a result that contains an error if `deps`
    /// is not empty.
    pub fn set_deps_lenient(&self, deps: D) -> Result<(), D> {
        self.deps.set(deps)
    }

    /// Sets the values of `cfg` and/or `deps` if either is empty, returns a result that contains an error if either`
    /// is not empty.
    pub fn set_lenient(&self, cfg: C, deps: D) -> Result<(), (Option<C>, Option<D>)> {
        let res_c = self.set_cfg_lenient(cfg).err();
        let res_d = self.set_deps_lenient(deps).err();
        if res_c.is_none() && res_d.is_none() {
            Ok(())
        } else {
            Err((res_c, res_d))
        }
    }

    /// Primes the instance by ensuring the `cfg` and `deps` values are set.
    /// Panics if `cfg` and/or `deps` is empty and there is no corresponding `cfg_init`/`deps_init`
    /// to set the value of `cfg` and/or `deps`.
    pub fn prime(&self) {
        self.get_cfg();
        self.get_deps();
    }

    ////
    // Helpers

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

impl<C> CfgDeps<C, ()> {
    /// Creates an uninitialized instance with with the specified value of `cfg_init` and `|| ()`for
    /// `deps_init` that will be available for lazy initialization.
    pub const fn lazy_init_with_cfg(cfg_init: fn() -> C) -> Self {
        Self::lazy_init(cfg_init, || ())
    }
}
