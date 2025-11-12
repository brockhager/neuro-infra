use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub node: NodeConfig,
    pub solana: SolanaConfig,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub program_id: String,
}