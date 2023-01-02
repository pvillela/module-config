use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppCfgInfo {
    pub x: String,
    pub y: i32,
}

pub fn getAppConfiguration() -> Arc<AppCfgInfo> {
    Arc::new(AppCfgInfo {
        x: "xxx".to_owned(),
        y: 42,
    })
}
