use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::Local;

pub fn safe_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn ensure_dir(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(path)?;
    Ok(path.to_path_buf())
}

pub fn today_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
