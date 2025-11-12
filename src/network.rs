use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use quinn::{ClientConfig, Endpoint, ServerConfig};
use rustls::Certificate;
use anyhow::Result;
use tracing::{info, warn};

pub struct Network {
    endpoint: Endpoint,
    peers: Arc<RwLock<HashMap<SocketAddr, Peer>>>,
    banlist: Arc<RwLock<HashMap<SocketAddr, std::time::Instant>>>,
    config: NetworkConfig,
}

#[derive(Clone)]
pub struct NetworkConfig {
    pub dns_seeds: Vec<String>,
    pub static_peers: Vec<String>,
    pub listen_addr: String,
    pub max_peers: usize,
}

#[derive(Debug)]
pub struct Peer {
    pub addr: SocketAddr,
    pub node_id: String,
    pub version: String,
}

impl Network {
    pub async fn new(config: NetworkConfig) -> Result<Self> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        let priv_key = rustls::PrivateKey(priv_key);
        let cert_chain = vec![Certificate(cert_der)];

        let server_config = ServerConfig::with_single_cert(cert_chain.clone(), priv_key)?;
        let mut endpoint = Endpoint::server(server_config, config.listen_addr.parse()?)?;

        let client_config = ClientConfig::with_custom_certificate(cert_chain)?;
        endpoint.set_default_client_config(client_config);

        Ok(Self {
            endpoint,
            peers: Arc::new(RwLock::new(HashMap::new())),
            banlist: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    pub async fn start(&self) -> Result<()> {
        // Resolve DNS seeds
        for seed in &self.config.dns_seeds {
            if let Ok(addrs) = tokio::net::lookup_host(seed).await {
                for addr in addrs {
                    self.connect_peer(addr).await?;
                }
            }
        }

        // Connect to static peers
        for peer in &self.config.static_peers {
            if let Ok(addr) = peer.parse() {
                self.connect_peer(addr).await?;
            }
        }

        // Listen for incoming connections
        while let Some(conn) = self.endpoint.accept().await {
            let peers = self.peers.clone();
            let banlist = self.banlist.clone();
            tokio::spawn(async move {
                if let Ok(conn) = conn.await {
                    // Handle handshake
                    if let Err(e) = handle_handshake(conn, peers, banlist).await {
                        warn!("Handshake failed: {:?}", e);
                    }
                }
            });
        }

        Ok(())
    }

    async fn connect_peer(&self, addr: SocketAddr) -> Result<()> {
        if self.peers.read().await.len() >= self.config.max_peers {
            return Ok(());
        }
        if self.banlist.read().await.contains_key(&addr) {
            return Ok(());
        }

        let conn = self.endpoint.connect(addr, "localhost")?.await?;
        let peers = self.peers.clone();
        let banlist = self.banlist.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_handshake(conn, peers, banlist).await {
                warn!("Outgoing handshake failed: {:?}", e);
            }
        });

        Ok(())
    }
}

async fn handle_handshake(
    conn: quinn::Connection,
    peers: Arc<RwLock<HashMap<SocketAddr, Peer>>>,
    banlist: Arc<RwLock<HashMap<SocketAddr, std::time::Instant>>>,
) -> Result<()> {
    let (mut send, mut recv) = conn.accept_bi().await?;

    // Send handshake
    let handshake = Handshake {
        node_id: "node123".to_string(),
        version: "0.1.0".to_string(),
    };
    let data = serde_json::to_vec(&handshake)?;
    send.write_all(&data).await?;
    send.finish().await?;

    // Receive handshake
    let mut buf = vec![0; 1024];
    let n = recv.read(&mut buf).await?.unwrap();
    let peer_handshake: Handshake = serde_json::from_slice(&buf[..n])?;

    if peer_handshake.version != "0.1.0" {
        // Ban for incompatible version
        banlist.write().await.insert(conn.remote_address(), std::time::Instant::now() + std::time::Duration::from_secs(3600));
        return Err(anyhow::anyhow!("Version mismatch"));
    }

    let peer = Peer {
        addr: conn.remote_address(),
        node_id: peer_handshake.node_id,
        version: peer_handshake.version,
    };
    peers.write().await.insert(conn.remote_address(), peer);
    info!("Connected to peer: {:?}", conn.remote_address());

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Handshake {
    node_id: String,
    version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handshake() {
        // Mock test for handshake
        let handshake = Handshake {
            node_id: "test".to_string(),
            version: "0.1.0".to_string(),
        };
        let data = serde_json::to_vec(&handshake).unwrap();
        let parsed: Handshake = serde_json::from_slice(&data).unwrap();
        assert_eq!(parsed.node_id, "test");
    }
}