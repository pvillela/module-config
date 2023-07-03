use cfgdepsobj_r::fs::boot::get_foo_sfl_d_cached;
use common::config::refresh_app_configuration;
use std::thread;

fn main() {
    let foo_sfl_d = get_foo_sfl_d_cached();

    let handle = thread::spawn(move || {
        let res = foo_sfl_d.run();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(100));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl_d.run();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(100));
        refresh_app_configuration();
        println!("App configuration refreshed -- output should be different.");

        let res = foo_sfl_d.run();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
