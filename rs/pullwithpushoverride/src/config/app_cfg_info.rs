use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppCfgInfo {
    x: String,
    y: i32,
}

pub fn getAppConfiguration() -> Arc<AppCfgInfo> {
    Arc::new(AppCfgInfo {
        x: "xxx".to_owned(),
        y: 42,
    })
}
