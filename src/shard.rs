use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShardReadReq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShardReadRes {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShardWriteReq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShardWriteRes {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shard {}

impl Rsm for Shard {
    type ReadReq = ShardReadReq;
    type ReadRes = ShardReadRes;
    type WriteReq = ShardWriteReq;
    type WriteRes = ShardWriteRes;

    fn read(&self, req: ShardReadReq) -> ShardReadRes {
        todo!()
    }

    fn write(&mut self, req: ShardWriteReq) -> ShardWriteRes {
        todo!()
    }

    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Shard> {
        todo!()
    }
}
