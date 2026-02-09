use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    if let Ok(path) = std::env::var("FERRUMBOT_DATA_DIR")
        && !path.trim().is_empty()
    {
        return expand_tilde(&path);
    }
    expand_tilde("~/.ferrum-bot")
}

pub fn config_path() -> PathBuf {
    data_dir().join("config.json")
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(path)
}
