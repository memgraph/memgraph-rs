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

    fn wrap(msg: RsmMessage<Shard>) -> Message {
        Message::Shard(msg)
    }

    fn unwrap(msg: Message) -> RsmMessage<Shard> {
        if let Message::Shard(wrapped) = msg {
            wrapped
        } else {
            unreachable!()
        }
    }
}
