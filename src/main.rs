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

    #[arg(long)]
    pub mode: Option<String>,

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
    let mut config: Config = serde_yaml::from_str(&config_content)?;

    // Override mode if specified
    if let Some(mode) = &args.mode {
        config.node.mode = mode.clone();
    }

    let mode = config.node.mode.as_str();

    match args.command {
        Some(Commands::Start) => {
            info!("Starting NeuroSwarm node in {} mode", mode);

            let network_config = NetworkConfig {
                dns_seeds: config.network.dns_seeds,
                static_peers: config.network.static_peers,
                listen_addr: config.network.listen_addr,
                max_peers: config.network.max_peers,
            };

            // Always start network for connectivity
            let network = Arc::new(Network::new(network_config).await?);
            let storage = Arc::new(Storage::new("catalog.db")?);

            // Load index for all modes that need it
            let mut index = Index::new();
            for manifest in storage.list_manifests()? {
                index.insert(manifest);
            }

            match mode {
                "validator" => {
                    // Validator: network, storage, anchor, sync
                    let anchor = Arc::new(Anchor::new(&config.solana.rpc_url, &config.solana.program_id)?);
                    let sync_engine = SyncEngine::new(storage.clone(), network.clone(), anchor.clone());
                    network.start().await?;
                    sync_engine.start_sync().await?;
                    info!("Validator mode: anchoring and consensus active");
                }
                "gateway" => {
                    // Gateway: network, storage, API server (future: start HTTP server)
                    network.start().await?;
                    info!("Gateway mode: API server active");
                    // TODO: Start HTTP server for API endpoints
                }
                "indexer" => {
                    // Indexer: network, storage, index, search APIs (future: start search server)
                    network.start().await?;
                    info!("Indexer mode: search and lineage active");
                    // TODO: Start search API server
                }
                "full" | _ => {
                    // Full node: all components
                    let anchor = Arc::new(Anchor::new(&config.solana.rpc_url, &config.solana.program_id)?);
                    let ipfs = IpfsCache::new();
                    let sync_engine = SyncEngine::new(storage.clone(), network.clone(), anchor.clone());
                    network.start().await?;
                    sync_engine.start_sync().await?;
                    info!("Full mode: all components active");
                }
            }
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