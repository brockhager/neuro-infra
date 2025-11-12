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
            "CREATE TABLE IF NOT EXISTS attestations (
                id INTEGER PRIMARY KEY,
                manifest_cid TEXT,
                validator TEXT,
                confidence REAL,
                timestamp INTEGER,
                FOREIGN KEY(manifest_cid) REFERENCES manifests(cid)
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

    pub fn stats(&self) -> Result<(usize, usize)> {
        let manifest_count: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM manifests",
            [],
            |row| row.get(0),
        )?;
        let attestation_count: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM attestations",
            [],
            |row| row.get(0),
        )?;
        Ok((manifest_count, attestation_count))
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