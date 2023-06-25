use crate::fs::{boot::get_foo_ai_sfl_wtih_app_cfg, FooAiSflT};
use common::config::get_app_configuration;

pub fn get_foo_ai_sfl() -> FooAiSflT {
    get_foo_ai_sfl_wtih_app_cfg(get_app_configuration)
}
