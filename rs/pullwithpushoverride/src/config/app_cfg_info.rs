#[derive(Debug)]
pub struct AppCfgInfo {
	x: String,
	y: i32,
}

pub fn getAppConfiguration() -> AppCfgInfo {
	AppCfgInfo {
		x: "xxx".to_owned(),
		y: 42,
	}
}
