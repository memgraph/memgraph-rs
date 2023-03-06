// Copyright 2023 Memgraph Ltd.
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt; by using this file, you agree to be bound by the terms of the Business Source
// License, and you may not use this file except in compliance with the Business Source License.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

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
