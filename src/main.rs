use clap::{Parser, Subcommand};
use std::fs;
use anyhow::Result;
use tracing_subscriber;

mod config;
mod network;

use config::Config;
use network::{Network, NetworkConfig};

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
}

#[derive(Subcommand)]
pub enum PeerCommands {
    Add { addr: String },
    List,
    Remove { addr: String },
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
            network.start().await?;
        }
        Some(Commands::Peer { peer_cmd }) => {
            match peer_cmd {
                PeerCommands::Add { addr } => {
                    println!("Adding peer: {}", addr);
                    // TODO: add to config
                }
                PeerCommands::List => {
                    println!("Listing peers");
                    // TODO: list from network
                }
                PeerCommands::Remove { addr } => {
                    println!("Removing peer: {}", addr);
                    // TODO: remove from config
                }
            }
        }
        _ => {
            println!("Use 'nsd start' to start the daemon");
        }
    }

    Ok(())
}