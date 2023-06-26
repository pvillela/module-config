# pushdepstovar_c

This package shows the implementation of fwk::cfg_deps using a stuct whose only field is a closure that contains as its state both the configuration cache and dependencies. That struct is wrapped in an ArcSwap and stored in a OnceLock static. This is in contrast with the struct that is used in common::fwk::cfg_deps, which contains an inner struct which in turn is wraped in ArcSwap or RefCell.
