use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorReadReq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorReadRes {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorWriteReq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorWriteRes {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinator {}

impl Rsm for Coordinator {
    type ReadReq = CoordinatorReadReq;
    type ReadRes = CoordinatorReadRes;
    type WriteReq = CoordinatorWriteReq;
    type WriteRes = CoordinatorWriteRes;

    fn read(&self, req: CoordinatorReadReq) -> CoordinatorReadRes {
        todo!()
    }

    fn write(&mut self, req: CoordinatorWriteReq) -> CoordinatorWriteRes {
        todo!()
    }

    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Coordinator> {
        todo!()
    }
}
