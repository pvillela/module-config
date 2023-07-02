use cfgdepsobj_r::fs::boot::get_foo_sfl_s_no_refresh;
use common::config::refresh_app_configuration;
use std::thread;

fn main() {
    let foo_sfl_s = get_foo_sfl_s_no_refresh();

    let handle = thread::spawn(move || {
        let res = foo_sfl_s.run();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl_s.run();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl_s.run();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
