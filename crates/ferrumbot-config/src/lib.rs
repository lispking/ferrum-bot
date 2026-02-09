mod paths;
mod schema;
mod storage;

pub const APP_NAME: &str = "ferrum-bot";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use paths::{config_path, data_dir, expand_tilde};
pub use schema::*;
pub use storage::{ensure_workspace_templates, load_config, save_config};
