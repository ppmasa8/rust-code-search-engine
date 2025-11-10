use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub data_dir: String,
    pub default_namespace: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: "./data".into(),
            default_namespace: "global".into(),
        }
    }
}

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
