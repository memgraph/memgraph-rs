use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type SchemaId = u64;
pub type EdgeLabelId = u64;
pub type VertexLabelId = u64;
pub type EdgePropertyId = u64;
pub type VertexPropertyId = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hlc {
    pub logical: u64,
    pub coordinator_wall: std::time::SystemTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionedKey {
    pub key: Key,
    pub version: Hlc,
}

// Key is a unified way of keying all data
// in the system. While it uses a Rust enum in this
// representation, it's just semantic sugar for a
// tuple of several fields, with some of them being
// optional, as the more complex ones are a superset
// of the less complex ones.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    Vertex {
        label: VertexLabelId,
        key: Vec<Value>,
    },
    VertexProperty {
        label: VertexLabelId,
        key: Vec<Value>,
        property: VertexPropertyId,
    },
    Edge {
        edge_label: EdgeLabelId,
        from_label: VertexLabelId,
        from_key: Vec<Value>,
        to_label: VertexLabelId,
        to_key: Vec<Value>,
    },
    EdgeProperty {
        edge_label: VertexLabelId,
        edge_property: EdgePropertyId,
        from_label: VertexLabelId,
        from_key: Vec<Value>,
        to_label: VertexLabelId,
        to_key: Vec<Value>,
    },
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchemaPart {
    #[default]
    Int,
    String,
    Map,
    List,
    Time,
}

pub type Schema = Vec<SchemaPart>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Int(i64),
    String(Box<str>),
    Map(BTreeMap<Key, Value>),
    List(Vec<Value>),
    Time(std::time::SystemTime),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    pub id: u64,
    pub ip_addr: std::net::IpAddr,
}
