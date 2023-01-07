use pushtofunction::config::app_cfg_info::refresh_app_configuration;
use pushtofunction::startup::init::{foo_sfl, foo_sfl1};

fn main() {
    foo_sfl();
    foo_sfl1();

    // Change of app config properties at runtime
    refresh_app_configuration();

    foo_sfl()
}
