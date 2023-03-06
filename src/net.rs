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

use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Message {
    CoordinatorAppendReq(AppendReq<Coordinator>),
    CoordinatorAppendRes(AppendRes<Coordinator>),
    CoordinatorReadReq(ReadReq<Coordinator>),
    CoordinatorReadRes(ReadRes<Coordinator>),
    CoordinatorWriteReq(WriteReq<Coordinator>),
    CoordinatorWriteRes(WriteRes<Coordinator>),
    ShardAppendReq(AppendReq<Shard>),
    ShardAppendRes(AppendRes<Shard>),
    ShardReadReq(ReadReq<Shard>),
    ShardReadRes(ReadRes<Shard>),
    ShardWriteReq(WriteReq<Shard>),
    ShardWriteRes(WriteRes<Shard>),
    VoteReq(VoteReq),
    VoteRes(VoteRes),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Envelope {
    from: Address,
    to: Address,
    message: Message,
    request_id: Option<u64>,
}
