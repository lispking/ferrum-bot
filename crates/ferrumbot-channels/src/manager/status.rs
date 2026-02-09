use std::collections::HashMap;

use super::ChannelManager;

impl ChannelManager {
    pub async fn status(&self) -> HashMap<String, bool> {
        let mut out = HashMap::new();
        for (name, channel) in &self.channels {
            out.insert(name.clone(), channel.is_running().await);
        }
        out
    }
}
