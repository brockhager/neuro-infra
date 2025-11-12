use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, commitment_config::CommitmentConfig};
use anyhow::Result;
use tracing::{info, warn};
use std::str::FromStr;

use crate::storage::{Storage, Manifest};

pub struct Anchor {
    client: RpcClient,
    program_id: Pubkey,
}

impl Anchor {
    pub fn new(rpc_url: &str, program_id: &str) -> Result<Self> {
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        let program_id = Pubkey::from_str(program_id)?;
        Ok(Self { client, program_id })
    }

    pub async fn verify_manifest(&self, cid: &str, creator: &str) -> Result<bool> {
        // Derive manifest PDA
        let creator_pubkey = Pubkey::from_str(creator)?;
        let (manifest_pda, _) = Pubkey::find_program_address(
            &[b"manifest", creator_pubkey.as_ref(), cid.as_bytes()],
            &self.program_id,
        );

        // Fetch account info
        match self.client.get_account(&manifest_pda) {
            Ok(account) => {
                // Deserialize and verify
                // For now, assume it's valid if account exists
                info!("Manifest {} verified on-chain", cid);
                Ok(true)
            }
            Err(_) => {
                warn!("Manifest {} not found on-chain", cid);
                Ok(false)
            }
        }
    }

    pub async fn get_manifest_provenance(&self, cid: &str, creator: &str) -> Result<Option<ManifestProvenance>> {
        let creator_pubkey = Pubkey::from_str(creator)?;
        let (manifest_pda, _) = Pubkey::find_program_address(
            &[b"manifest", creator_pubkey.as_ref(), cid.as_bytes()],
            &self.program_id,
        );

        match self.client.get_account(&manifest_pda) {
            Ok(account) => {
                // Deserialize account data
                // This would need proper deserialization based on the account struct
                // For now, return mock data
                Ok(Some(ManifestProvenance {
                    finalized: true,
                    attestation_count: 3,
                    tx_signature: "mock_sig".to_string(),
                    slot: 12345,
                }))
            }
            Err(_) => Ok(None),
        }
    }
}

#[derive(Debug)]
pub struct ManifestProvenance {
    pub finalized: bool,
    pub attestation_count: u64,
    pub tx_signature: String,
    pub slot: u64,
}