use crate::fs::boot::foo_ai_sfl_boot;
use crate::fs::FooAiSflT;
use common::config::get_app_configuration;

pub fn make_foo_ai_sfl() -> Box<FooAiSflT> {
    foo_ai_sfl_boot(get_app_configuration)
}
