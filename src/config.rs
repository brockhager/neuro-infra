use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub node: NodeConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub dns_seeds: Vec<String>,
    pub static_peers: Vec<String>,
    pub listen_addr: String,
    pub max_peers: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    pub node_id: String,
    pub version: String,
}