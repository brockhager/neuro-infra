use clap::{Parser, Subcommand};
use std::fs;
use std::sync::Arc;
use anyhow::Result;
use tracing_subscriber;

mod config;
mod network;
mod storage;
mod ipfs;
mod index;
mod sync;
mod anchor;

use config::Config;
use network::{Network, NetworkConfig};
use storage::Storage;
use ipfs::IpfsCache;
use index::Index;
use sync::SyncEngine;
use anchor::Anchor;
use sync::Sync;
use anchor::Anchor;

#[derive(Parser)]
#[command(name = "nsd")]
#[command(about = "NeuroSwarm Node Daemon")]
pub struct Args {
    #[arg(short, long, default_value = "~/.neuroswarm/ns.conf")]
    pub config: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Stop,
    Status,
    Peer {
        #[command(subcommand)]
        peer_cmd: PeerCommands,
    },
    Catalog {
        #[command(subcommand)]
        catalog_cmd: CatalogCommands,
    },
}

#[derive(Subcommand)]
pub enum PeerCommands {
    Add { addr: String },
    List,
    Remove { addr: String },
}

#[derive(Subcommand)]
pub enum CatalogCommands {
    List,
    Prune { days: u32 },
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();

    let args = Args::parse();

    let config_content = fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    match args.command {
        Some(Commands::Start) => {
            let network_config = NetworkConfig {
                dns_seeds: config.network.dns_seeds,
                static_peers: config.network.static_peers,
                listen_addr: config.network.listen_addr,
                max_peers: config.network.max_peers,
            };
            let network = Network::new(network_config).await?;
            let storage = Arc::new(Storage::new("catalog.db")?);
            let anchor = Arc::new(Anchor::new(&config.solana.rpc_url, &config.solana.program_id)?);
            let ipfs = IpfsCache::new();
            let mut index = Index::new();
            // Load existing manifests into index
            for manifest in storage.list_manifests()? {
                index.insert(manifest);
            }
            let sync_engine = SyncEngine::new(storage.clone(), network.clone(), anchor.clone());
            network.start().await?;
            sync_engine.start_sync().await?;
        }
        Some(Commands::Catalog { catalog_cmd }) => {
            let storage = Storage::new("catalog.db")?;
            match catalog_cmd {
                CatalogCommands::List => {
                    let manifests = storage.list_manifests()?;
                    for m in manifests {
                        println!("CID: {}", m.cid);
                    }
                }
                CatalogCommands::Prune { days } => {
                    let before = chrono::Utc::now().timestamp() - (days as i64 * 86400);
                    let count = storage.prune_old(before)?;
                    println!("Pruned {} manifests", count);
                }
                CatalogCommands::Stats => {
                    let (m_count, a_count) = storage.stats()?;
                    println!("Manifests: {}, Attestations: {}", m_count, a_count);
                }
            }
        }
        _ => {
            println!("Use 'nsd start' to start the daemon");
        }
    }

    Ok(())
}