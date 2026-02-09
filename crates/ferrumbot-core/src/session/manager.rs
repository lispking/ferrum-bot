use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use ferrumbot_config::data_dir;

use super::Session;
use super::io::{load_session, save_session};
use crate::safe_filename;

pub struct SessionManager {
    sessions_dir: PathBuf,
    cache: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let sessions_dir = data_dir().join("sessions");
        fs::create_dir_all(&sessions_dir)?;
        Ok(Self {
            sessions_dir,
            cache: HashMap::new(),
        })
    }

    pub fn get_or_create(&mut self, key: &str) -> Result<&mut Session> {
        if !self.cache.contains_key(key) {
            let session = self
                .load(key)?
                .unwrap_or_else(|| Session::new(key.to_string()));
            self.cache.insert(key.to_string(), session);
        }
        Ok(self.cache.get_mut(key).expect("session inserted"))
    }

    pub fn save(&mut self, key: &str) -> Result<()> {
        let Some(session) = self.cache.get(key) else {
            return Ok(());
        };

        let path = self.session_path(key);
        save_session(&path, session)
    }

    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.sessions_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("jsonl")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            {
                out.push(stem.replace('_', ":"));
            }
        }
        out.sort();
        Ok(out)
    }

    fn load(&self, key: &str) -> Result<Option<Session>> {
        let path = self.session_path(key);
        load_session(&path, key)
    }

    fn session_path(&self, key: &str) -> PathBuf {
        let safe = safe_filename(&key.replace(':', "_"));
        self.sessions_dir.join(format!("{safe}.jsonl"))
    }
}
