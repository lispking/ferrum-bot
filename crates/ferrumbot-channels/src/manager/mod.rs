use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use ferrumbot_core::MessageBus;

use crate::BaseChannel;

pub struct ChannelManager {
    pub(super) bus: MessageBus,
    pub(super) channels: HashMap<String, Arc<dyn BaseChannel>>,
    pub(super) dispatch_task: Arc<RwLock<Option<JoinHandle<()>>>>,
}

mod build;
mod lifecycle;
mod status;
