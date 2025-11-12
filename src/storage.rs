use rusqlite::{params, Connection, Result};
use std::path::Path;
use tracing::{info, warn};

pub struct Storage {
    conn: Connection,
}

#[derive(Debug)]
pub struct Manifest {
    pub cid: String,
    pub data: Vec<u8>,
    pub timestamp: i64,
}

#[derive(Debug)]
pub struct Provenance {
    pub finalized: bool,
    pub attestation_count: u64,
    pub tx_signature: String,
    pub slot: u64,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS manifests (
                cid TEXT PRIMARY KEY,
                data BLOB,
                timestamp INTEGER
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS provenance (
                cid TEXT PRIMARY KEY,
                finalized BOOLEAN,
                attestation_count INTEGER,
                tx_signature TEXT,
                slot INTEGER,
                cached_at INTEGER,
                FOREIGN KEY(cid) REFERENCES manifests(cid)
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert_manifest(&self, manifest: &Manifest) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO manifests (cid, data, timestamp) VALUES (?1, ?2, ?3)",
            params![manifest.cid, manifest.data, manifest.timestamp],
        )?;
        info!("Inserted manifest: {}", manifest.cid);
        Ok(())
    }

    pub fn get_manifest(&self, cid: &str) -> Result<Option<Manifest>> {
        let mut stmt = self.conn.prepare("SELECT cid, data, timestamp FROM manifests WHERE cid = ?1")?;
        let mut rows = stmt.query_map(params![cid], |row| {
            Ok(Manifest {
                cid: row.get(0)?,
                data: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;
        if let Some(manifest) = rows.next() {
            Ok(Some(manifest?))
        } else {
            Ok(None)
        }
    }

    pub fn list_manifests(&self) -> Result<Vec<Manifest>> {
        let mut stmt = self.conn.prepare("SELECT cid, data, timestamp FROM manifests")?;
        let rows = stmt.query_map([], |row| {
            Ok(Manifest {
                cid: row.get(0)?,
                data: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;
        let mut manifests = Vec::new();
        for manifest in rows {
            manifests.push(manifest?);
        }
        Ok(manifests)
    }

    pub fn prune_old(&self, before_timestamp: i64) -> Result<usize> {
        let count = self.conn.execute(
            "DELETE FROM manifests WHERE timestamp < ?1",
            params![before_timestamp],
        )?;
        warn!("Pruned {} old manifests", count);
        Ok(count)
    }

    pub fn cache_provenance(&self, cid: &str, provenance: &Provenance) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO provenance (cid, finalized, attestation_count, tx_signature, slot, cached_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![cid, provenance.finalized, provenance.attestation_count, provenance.tx_signature, provenance.slot, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }

    pub fn get_provenance(&self, cid: &str) -> Result<Option<Provenance>> {
        let mut stmt = self.conn.prepare("SELECT finalized, attestation_count, tx_signature, slot FROM provenance WHERE cid = ?1")?;
        let mut rows = stmt.query_map(params![cid], |row| {
            Ok(Provenance {
                finalized: row.get(0)?,
                attestation_count: row.get(1)?,
                tx_signature: row.get(2)?,
                slot: row.get(3)?,
            })
        })?;
        if let Some(provenance) = rows.next() {
            Ok(Some(provenance?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_storage() -> Result<()> {
        let db_path = "test.db";
        let storage = Storage::new(db_path)?;
        let manifest = Manifest {
            cid: "test_cid".to_string(),
            data: vec![1, 2, 3],
            timestamp: 123456,
        };
        storage.insert_manifest(&manifest)?;
        let retrieved = storage.get_manifest("test_cid")?;
        assert_eq!(retrieved.unwrap().cid, "test_cid");
        fs::remove_file(db_path)?;
        Ok(())
    }
}