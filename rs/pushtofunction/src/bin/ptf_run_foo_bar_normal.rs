use pushtofunction::config::refresh_app_configuration;
use pushtofunction::startup::{make_foo_sfl, make_foo_sfl1};

fn main() {
    let foo_sfl = make_foo_sfl();
    let foo_sfl1 = make_foo_sfl1();

    foo_sfl();
    foo_sfl1();

    // Change of app config properties at runtime
    refresh_app_configuration();

    foo_sfl();
}
