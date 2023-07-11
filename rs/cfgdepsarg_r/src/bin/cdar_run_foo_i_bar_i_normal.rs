use cfgdepsarg_r::startup::make_foo_i_sfl;
use common::config::refresh_app_configuration;

fn main() {
    let foo_i_sfl = make_foo_i_sfl();
    let res = foo_i_sfl(());
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let res = foo_i_sfl(());
    println!("{}", res);
}
