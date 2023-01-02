use pullwithpushoverride::getAppConfiguration;

fn main() {
    // let plant = Asparagus {};
    // println!("I'm growing {:?}!", plant);

    let app_cfg = getAppConfiguration();
    println!("AppConfiguration: {:?}", app_cfg);
}
