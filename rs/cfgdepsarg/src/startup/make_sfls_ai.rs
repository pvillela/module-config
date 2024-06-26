use std::sync::OnceLock;

use crate::fs;
use crate::fs::{foo_ai_sfl_boot_lr, FooAiSflT};
use common::config::get_app_configuration;

pub fn make_foo_ai_sfl() -> Box<FooAiSflT> {
    fs::foo_ai_sfl_boot(&get_app_configuration())
}

pub fn get_foo_ai_sfl() -> &'static FooAiSflT {
    static FOO_AI_SFL: OnceLock<&FooAiSflT> = OnceLock::new();
    FOO_AI_SFL.get_or_init(|| foo_ai_sfl_boot_lr(&get_app_configuration()))
}
