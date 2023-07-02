use cfgdepsmethods::startup::get_foo_sfl_no_refresh;
use common::config::refresh_app_configuration;
use std::thread;

fn main() {
    let foo_sfl = get_foo_sfl_no_refresh();

    let handle = thread::spawn(move || {
        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
