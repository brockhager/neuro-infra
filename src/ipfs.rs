use ipfs_api::IpfsClient;
use std::io::Cursor;
use anyhow::Result;
use tracing::info;

pub struct IpfsCache {
    client: IpfsClient,
}

impl IpfsCache {
    pub fn new() -> Self {
        Self {
            client: IpfsClient::default(),
        }
    }

    pub async fn pin(&self, cid: &str) -> Result<()> {
        self.client.pin_add(cid, true).await?;
        info!("Pinned CID: {}", cid);
        Ok(())
    }

    pub async fn unpin(&self, cid: &str) -> Result<()> {
        self.client.pin_rm(cid, true).await?;
        info!("Unpinned CID: {}", cid);
        Ok(())
    }

    pub async fn add_data(&self, data: &[u8]) -> Result<String> {
        let cursor = Cursor::new(data);
        let res = self.client.add(cursor).await?;
        let cid = res.hash;
        info!("Added data to IPFS: {}", cid);
        Ok(cid)
    }

    pub async fn get_data(&self, cid: &str) -> Result<Vec<u8>> {
        let data = self.client.cat(cid).await?;
        Ok(data.to_vec())
    }
}