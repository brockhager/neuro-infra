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
    Catalog {
        #[command(subcommand)]
        catalog_cmd: CatalogCommands,
    },
    Peer {
        #[command(subcommand)]
        peer_cmd: PeerCommands,
    },
    Index {
        #[command(subcommand)]
        index_cmd: IndexCommands,
    },
    Anchor {
        #[command(subcommand)]
        anchor_cmd: AnchorCommands,
    },
}

#[derive(Subcommand)]
pub enum CatalogCommands {
    List,
    Prune { days: u32 },
    Stats,
}

#[derive(Subcommand)]
pub enum PeerCommands {
    Add { addr: String },
    List,
    Remove { addr: String },
}

#[derive(Subcommand)]
pub enum IndexCommands {
    Search { query: Option<String>, tag: Option<String> },
    Lineage { cid: String },
    Confidence { cid: String },
}

#[derive(Subcommand)]
pub enum AnchorCommands {
    Verify { cid: String, creator: String },
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
        Some(Commands::Index { index_cmd }) => {
            let index = Index::new();
            // Load from storage
            let storage = Storage::new("catalog.db")?;
            for manifest in storage.list_manifests()? {
                index.insert(manifest);
            }
            match index_cmd {
                IndexCommands::Search { query, tag } => {
                    // Mock search
                    println!("Search results for query: {:?}, tag: {:?}", query, tag);
                }
                IndexCommands::Lineage { cid } => {
                    println!("Lineage for {}: Mock lineage data", cid);
                }
                IndexCommands::Confidence { cid } => {
                    println!("Confidence for {}: Mock confidence score", cid);
                }
            }
        }
        Some(Commands::Anchor { anchor_cmd }) => {
            let anchor = Anchor::new("https://api.devnet.solana.com", "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")?;
            match anchor_cmd {
                AnchorCommands::Verify { cid, creator } => {
                    let verified = anchor.verify_manifest(&cid, &creator).await?;
                    println!("Verification result for {}: {}", cid, verified);
                }
            }
        }
    }

    Ok(())
}