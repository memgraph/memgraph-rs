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
use std::marker::PhantomData;
use std::path::Path;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{Address, Io, Message};

pub trait Rsm: Sized + Serialize + for<'a> Deserialize<'a> {
    type ReadReq: Serialize + for<'a> Deserialize<'a>;
    type ReadRes: Serialize + for<'a> Deserialize<'a>;
    type WriteReq: Serialize + for<'a> Deserialize<'a>;
    type WriteRes: Serialize + for<'a> Deserialize<'a>;

    fn read(&self, request: Self::ReadReq) -> Self::ReadRes;
    fn write(&mut self, request: Self::WriteReq) -> Self::WriteRes;
    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Self>;
    fn wrap(msg: RsmMessage<Self>) -> Message;
    fn unwrap(msg: Message) -> RsmMessage<Self>;
}

pub struct RsmClient<R: Rsm> {
    pub timeout: std::time::Duration,
    pub retries: usize,
    pub peers: Vec<Address>,
    pub leader: Address,
    pub pd: PhantomData<R>,
    pub io: Io,
}

impl<R: Rsm> RsmClient<R> {
    async fn read(&mut self, req: R::ReadReq) -> io::Result<R::ReadRes> {
        todo!()
    }

    async fn write(&mut self, req: R::WriteReq) -> io::Result<R::WriteRes> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Term(pub u64);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RsmMessage<R: Rsm> {
    AppendReq(AppendReq<R::WriteReq>),
    AppendRes(AppendRes),
    ReadReq(ReadReq<R::ReadReq>),
    ReadRes(ReadRes<R::ReadRes>),
    WriteReq(WriteReq<R::WriteReq>),
    WriteRes(WriteRes<R::WriteRes>),
    VoteReq(VoteReq),
    VoteRes(VoteRes),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VoteReq {
    pub term: Term,
    pub log_size: u64,
    pub last_log_term: Term,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VoteRes {
    pub success: bool,
    pub term: Term,
    pub log_size: u64,
    pub last_log_term: Term,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteReq<Req> {
    req: Req,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WriteRes<Res> {
    Response(Res),
    Redirect(Address),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadReq<Req> {
    req: Req,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadRes<Res> {
    Response(Res),
    Redirect(Address),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendReq<Req> {
    term: Term,
    batch_start_log_index: u64,
    last_log_term: Term,
    entries: Vec<(Term, Req)>,
    leader_commit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendRes {
    success: bool,
    term: Term,
    last_log_term: Term,
    // a small optimization over the raft paper, tells
    // the leader the offset that we are interested in
    // to send log offsets from for us. This will only
    // be useful at the beginning of a leader's term.
    log_size: u64,
}

#[derive(Debug, Clone)]
pub struct Replica<R: Rsm> {
    state: R,
    io: Io,
}

impl<R: Rsm> Replica<R> {
    fn cron(&mut self) {
        // return early unless leader

        // handle timeouts

        // send Appends
        todo!()
    }

    fn handle_read(&self, req: ReadReq<R>) -> R::ReadRes {
        todo!()
    }

    fn handle_write(&mut self, req: WriteReq<R>) {
        todo!()
    }

    fn handle_append_req(&mut self, req: AppendReq<R>) {
        todo!()
    }

    fn handle_append_res(&mut self, req: AppendRes) {
        todo!()
    }

    fn handle_vote_req(&mut self, req: VoteReq) {
        todo!()
    }

    fn handle_vote_res(&mut self, req: VoteRes) {
        todo!()
    }
}
