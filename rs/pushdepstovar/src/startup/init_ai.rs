use crate::fs::{get_foo_ai_sfl_with_app_cfg, FooAiSflT};
use common::config::get_app_configuration;

pub fn get_foo_ai_sfl() -> FooAiSflT {
    get_foo_ai_sfl_with_app_cfg(get_app_configuration)
}

pub fn init_foo_ai_sfl() {
    get_foo_ai_sfl();
}
