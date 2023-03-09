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

use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{Address, Envelope, Io, Message};

pub trait Rsm: Sized + Serialize + for<'a> Deserialize<'a> {
    type ReadReq: fmt::Debug + Serialize + for<'a> Deserialize<'a>;
    type ReadRes: fmt::Debug + Serialize + for<'a> Deserialize<'a>;
    type WriteReq: fmt::Debug + Serialize + for<'a> Deserialize<'a>;
    type WriteRes: fmt::Debug + Serialize + for<'a> Deserialize<'a>;

    fn read(&self, request: Self::ReadReq) -> Self::ReadRes;
    fn write(&mut self, request: Self::WriteReq) -> Self::WriteRes;
    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Self>;
    fn wrap(msg: RsmMessage<Self>) -> Message;
    fn unwrap(msg: Message) -> Result<RsmMessage<Self>, Message>;
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
    async fn read(&mut self, req: R::ReadReq) -> io::Result<RsmResult<R::ReadRes>> {
        todo!()
    }

    async fn write(&mut self, req: R::WriteReq) -> io::Result<RsmResult<R::WriteRes>> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Redirect {
    leader: Address,
}

pub type RsmResult<R> = Result<R, Redirect>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Redirect { leader: Address, term: Term },
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
enum Role {
    Candidate,
    Leader,
    Follower { leader: Address, term: Term },
}

// This is a macro so that it can cause an early-return
// in the context that it is expanded in.
macro_rules! maybe_redirect_or_drop {
    ($self:expr, $from:expr, $request_id:expr) => {
        match &$self.role {
            Role::Candidate => return,
            Role::Follower { leader, term } => {
                let redirect_msg = R::wrap(RsmMessage::Redirect {
                    leader: *leader,
                    term: *term,
                });
                $self.io.send($from, $request_id, redirect_msg);

                return;
            }
            Role::Leader => {}
        }
    };
}

#[derive(Debug, Clone)]
pub struct Replica<R: Rsm> {
    state: R,
    io: Io,
    role: Role,
}

impl<R: Rsm> Replica<R> {
    fn cron(&mut self) {
        // return early unless leader

        // handle timeouts

        // send Appends
        todo!()
    }

    fn receive(&mut self, envelope: Envelope) {
        match R::unwrap(envelope.message) {
            Ok(RsmMessage::AppendReq(append_req)) => self.handle_append_req(append_req),
            Ok(RsmMessage::AppendRes(append_res)) => self.handle_append_res(append_res),
            Ok(RsmMessage::ReadReq(read_req)) => {
                self.handle_read_req(envelope.from, envelope.request_id, read_req)
            }
            Ok(RsmMessage::WriteReq(write_req)) => {
                self.handle_write_req(envelope.from, envelope.request_id, write_req)
            }
            Ok(RsmMessage::VoteReq(vote_req)) => self.handle_vote_req(vote_req),
            Ok(RsmMessage::VoteRes(vote_res)) => self.handle_vote_res(vote_res),
            Ok(RsmMessage::ReadRes(unexpected)) => {
                panic!("received unexpected message: {:?}", unexpected);
            }
            Ok(RsmMessage::WriteRes(unexpected)) => {
                panic!("received unexpected message: {:?}", unexpected);
            }
            Ok(RsmMessage::Redirect { .. }) => {
                panic!("received unexpected Redirect message");
            }
            Err(unexpected) => {
                panic!("received unexpected message: {:?}", unexpected);
            }
        }
    }

    fn handle_read_req(
        &mut self,
        from: Address,
        request_id: Option<u64>,
        req: ReadReq<R::ReadReq>,
    ) {
        maybe_redirect_or_drop!(self, from, request_id);
        todo!()
    }

    fn handle_write_req(
        &mut self,
        from: Address,
        request_id: Option<u64>,
        req: WriteReq<R::WriteReq>,
    ) {
        maybe_redirect_or_drop!(self, from, request_id);
        todo!()
    }

    fn handle_append_req(&mut self, req: AppendReq<R::WriteReq>) {
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
