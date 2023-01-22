pub mod config {
    mod app_cfg_info;
    pub use app_cfg_info::*;
}

pub mod fs {
    mod bar_bf;
    pub use bar_bf::*;

    pub mod boot {
        mod bar_bf_boot;
        pub use bar_bf_boot::*;

        mod foo_sfl_boot;
        pub use foo_sfl_boot::*;
    }

    mod foo_sfl;
    pub use foo_sfl::*;
}

pub mod fwk {
    mod cfg_adapter;
    pub use cfg_adapter::*;
}

pub mod startup {
    mod init;
    pub use init::*;
}
