use pushtofunction::config::refresh_app_configuration;
use pushtofunction::startup::{foo_sfl, foo_sfl1};

fn main() {
    foo_sfl();
    foo_sfl1();

    // Change of app config properties at runtime
    refresh_app_configuration();

    foo_sfl()
}
