use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use crate::memory::MemoryStore;

pub struct ServerState {
    pub memory_store: Arc<MemoryStore>,
    pub shutdown_tx: Mutex<Option<oneshot::Sender<()>>>,
}
