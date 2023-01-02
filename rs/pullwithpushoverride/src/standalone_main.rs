// use crate::garden::vegetables::Asparagus;
use crate::config::app_cfg_info::getAppConfiguration;

// pub mod garden;
pub mod config;

fn main() {
    // let plant = Asparagus {};
    // println!("I'm growing {:?}!", plant);

    let app_cfg = getAppConfiguration();
    println!("AppConfiguration: {:?}", app_cfg);
}
