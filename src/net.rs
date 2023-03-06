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
