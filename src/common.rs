use std::collections::{BTreeMap, HashMap, HashSet};
use std::num::NonZeroU64;
use std::ops::Bound;

use serde::{Deserialize, Serialize};

type PropertyId = u64;
type EdgeLabelId = u64;
type VertexLabelId = u64;
type EdgePropertyId = u64;
type VertexPropertyId = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hlc {
    logical: u64,
    coordinator_wall: std::time::SystemTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionedKey {
    key: Key,
    version: Hlc,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Int(i64),
    String(String),
    Map(BTreeMap<Key, Value>),
    List(Vec<Value>),
}
