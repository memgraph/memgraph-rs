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

// TODO(gitbuda): Remove, just to make project compile.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardMap {
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorReadReq {
    GetShardMap,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorReadRes {
    GetShardMap(ShardMap),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorWriteReq {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoordinatorWriteRes {}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinator {
    shard_map: ShardMap,
}

impl Rsm for Coordinator {
    type ReadReq = CoordinatorReadReq;
    type ReadRes = CoordinatorReadRes;
    type WriteReq = CoordinatorWriteReq;
    type WriteRes = CoordinatorWriteRes;

    fn read(&self, req: CoordinatorReadReq) -> CoordinatorReadRes {
        match req {
            CoordinatorReadReq::GetShardMap => {
                CoordinatorReadRes::GetShardMap(self.shard_map.clone())
            }
        }
    }

    fn write(&mut self, req: &CoordinatorWriteReq) -> CoordinatorWriteRes {
        todo!()
    }

    fn wrap(msg: RsmMessage<Coordinator>) -> Message {
        Message::Coordinator(msg)
    }

    fn unwrap(msg: Message) -> Result<RsmMessage<Coordinator>, Message> {
        if let Message::Coordinator(wrapped) = msg {
            Ok(wrapped)
        } else {
            Err(msg)
        }
    }
}
