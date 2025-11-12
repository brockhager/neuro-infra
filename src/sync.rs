use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, warn, error};
use crate::network::SyncMessage;

use crate::storage::{Storage, Manifest};
use crate::network::{Peer, SyncMessage, Network};

pub struct SyncEngine {
    storage: Arc<Storage>,
    network: Arc<Network>,
    last_sync: Arc<RwLock<HashMap<std::net::SocketAddr, i64>>>,
}

impl SyncEngine {
    pub fn new(storage: Arc<Storage>, network: Arc<Network>) -> Self {
        Self {
            storage,
            network,
            last_sync: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_sync(&self) -> Result<()> {
        let peers = self.network.peers.read().await.clone();
        for (addr, _peer) in peers {
            if let Err(e) = self.sync_with_peer(addr).await {
                warn!("Failed to sync with peer {}: {:?}", addr, e);
            }
        }
        Ok(())
    }

    async fn sync_with_peer(&self, addr: std::net::SocketAddr) -> Result<()> {
        let last_sync = *self.last_sync.read().await.get(&addr).unwrap_or(&0);
        let request = SyncMessage::RequestCatalog { since: Some(last_sync) };
        self.network.send_sync_message(addr, &request).await?;
        // TODO: Handle response - for now assume it works
        self.last_sync.write().await.insert(addr, chrono::Utc::now().timestamp());
        info!("Requested sync from peer: {}", addr);
        Ok(())
    }

    pub async fn handle_sync_message(&self, addr: std::net::SocketAddr, message: SyncMessage) -> Result<Option<SyncMessage>> {
        match message {
            SyncMessage::RequestCatalog { since } => {
                let manifests = self.storage.list_manifests()?;
                let filtered: Vec<Manifest> = if let Some(ts) = since {
                    manifests.into_iter().filter(|m| m.timestamp > ts).collect()
                } else {
                    manifests
                };
                // Send in chunks if large
                let chunk = filtered.into_iter().take(100).collect::<Vec<_>>();
                let has_more = chunk.len() == 100;
                Ok(Some(SyncMessage::CatalogChunk { manifests: chunk, has_more }))
            }
            SyncMessage::CatalogChunk { manifests, has_more: _ } => {
                for manifest in manifests {
                    if let Err(e) = self.storage.insert_manifest(&manifest) {
                        error!("Failed to insert manifest {}: {:?}", manifest.cid, e);
                    }
                }
                Ok(None)
            }
            SyncMessage::RequestManifest { cid } => {
                if let Some(manifest) = self.storage.get_manifest(&cid)? {
                    Ok(Some(SyncMessage::ManifestData { cid, data: manifest.data }))
                } else {
                    Ok(None)
                }
            }
            SyncMessage::ManifestData { cid: _, data: _ } => {
                // Store the data if needed
                Ok(None)
            }
        }
    }
}