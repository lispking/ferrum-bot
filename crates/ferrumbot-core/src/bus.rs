use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::{Mutex, mpsc};

use crate::{InboundMessage, OutboundMessage};

#[derive(Clone)]
pub struct MessageBus {
    inbound_tx: mpsc::Sender<InboundMessage>,
    outbound_tx: mpsc::Sender<OutboundMessage>,
    inbound_rx: Arc<Mutex<mpsc::Receiver<InboundMessage>>>,
    outbound_rx: Arc<Mutex<mpsc::Receiver<OutboundMessage>>>,
}

impl MessageBus {
    pub fn new(buffer: usize) -> Self {
        let (inbound_tx, inbound_rx) = mpsc::channel(buffer);
        let (outbound_tx, outbound_rx) = mpsc::channel(buffer);
        Self {
            inbound_tx,
            outbound_tx,
            inbound_rx: Arc::new(Mutex::new(inbound_rx)),
            outbound_rx: Arc::new(Mutex::new(outbound_rx)),
        }
    }

    pub async fn publish_inbound(&self, msg: InboundMessage) -> Result<()> {
        self.inbound_tx
            .send(msg)
            .await
            .context("failed to publish inbound message")
    }

    pub async fn consume_inbound(&self) -> Option<InboundMessage> {
        self.inbound_rx.lock().await.recv().await
    }

    pub async fn publish_outbound(&self, msg: OutboundMessage) -> Result<()> {
        self.outbound_tx
            .send(msg)
            .await
            .context("failed to publish outbound message")
    }

    pub async fn consume_outbound(&self) -> Option<OutboundMessage> {
        self.outbound_rx.lock().await.recv().await
    }
}
