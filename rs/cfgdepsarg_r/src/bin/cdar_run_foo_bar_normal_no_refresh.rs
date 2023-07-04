use cfgdepsarg_r::startup::make_foo_sfl_no_refresh;
use common::config::refresh_app_configuration;

fn main() {
    let foo_sfl = make_foo_sfl_no_refresh();
    let res = foo_sfl();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let res = foo_sfl();
    println!("{}", res);
}
