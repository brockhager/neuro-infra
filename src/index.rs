use std::collections::HashMap;
use crate::storage::Manifest;

pub struct Index {
    cid_index: HashMap<String, Manifest>,
    node_index: HashMap<String, Vec<String>>, // node_id -> cids
}

impl Index {
    pub fn new() -> Self {
        Self {
            cid_index: HashMap::new(),
            node_index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, manifest: Manifest) {
        let cid = manifest.cid.clone();
        self.cid_index.insert(cid.clone(), manifest);
        // For simplicity, assume node_id from manifest data
        // In real impl, parse manifest
        let node_id = "node123".to_string(); // placeholder
        self.node_index.entry(node_id).or_insert(Vec::new()).push(cid);
    }

    pub fn get_by_cid(&self, cid: &str) -> Option<&Manifest> {
        self.cid_index.get(cid)
    }

    pub fn get_by_node(&self, node_id: &str) -> Option<&Vec<String>> {
        self.node_index.get(node_id)
    }

    pub fn lineage(&self, cid: &str) -> Vec<String> {
        // Placeholder: return related cids
        vec![cid.to_string()]
    }
}