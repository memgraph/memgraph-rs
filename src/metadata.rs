use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMetadata {
    pub address: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardMetadata {
    pub low_key: Key,
    pub high_key: Option<Key>,
    pub shard_version: u64,
    pub peers: Vec<PeerMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaMetadata {
    pub schema: Schema,
    pub shards: BTreeMap<Key, ShardMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterMetadata {
    pub version: Hlc,
    pub coordinators: Vec<Address>,
    pub name_to_id: BTreeMap<String, u64>,
    pub id_to_name: BTreeMap<u64, String>,
    pub schemas: BTreeMap<SchemaId, SchemaMetadata>,
}

impl ClusterMetadata {
    fn idgen(&mut self) -> u64 {
        self.version.logical += 1;
        self.version.logical
    }
}
