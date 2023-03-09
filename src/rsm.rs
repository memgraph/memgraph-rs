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

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;
use std::marker::PhantomData;
use std::path::Path;
use std::time::{Duration, SystemTime};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{Address, Envelope, Io, Message};

pub trait Rsm: Sized + Serialize + for<'a> Deserialize<'a> {
    type ReadReq: fmt::Debug + Serialize + for<'a> Deserialize<'a>;
    type ReadRes: fmt::Debug + Serialize + for<'a> Deserialize<'a>;
    type WriteReq: fmt::Debug + Clone + Serialize + for<'a> Deserialize<'a>;
    type WriteRes: fmt::Debug + Serialize + for<'a> Deserialize<'a>;

    fn read(&self, request: Self::ReadReq) -> Self::ReadRes;
    fn write(&mut self, request: Self::WriteReq) -> Self::WriteRes;
    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Self>;
    fn wrap(msg: RsmMessage<Self>) -> Message;
    fn unwrap(msg: Message) -> Result<RsmMessage<Self>, Message>;
}

pub struct RsmClient<R: Rsm> {
    pub timeout: Duration,
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

impl Term {
    #[must_use]
    fn increment(&self) -> Term {
        Term(self.0 + 1)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RsmMessage<R: Rsm> {
    AppendReq(AppendReq<R::WriteReq>),
    AppendRes(AppendRes),
    ReadReq(R::ReadReq),
    ReadRes(R::ReadRes),
    WriteReq(R::WriteReq),
    WriteRes(R::WriteRes),
    VoteReq {
        proposed_leadership_term: Term,
        last_log_term: Option<Term>,
        committed_log_size: u64,
    },
    VoteRes {
        success: bool,
        term: Term,
        committed_log_size: u64,
    },
    Redirect {
        leader: Address,
        term: Term,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendReq<Req> {
    term: Term,
    batch_start_log_index: u64,
    last_log_term: Option<Term>,
    entries: Vec<(Term, Req)>,
    leader_committed_log_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendRes {
    success: bool,
    term: Term,
    last_log_term: Option<Term>,
    // a small optimization over the raft paper, tells
    // the leader the offset that we are interested in
    // to send log offsets from for us. This will only
    // be useful at the beginning of a leader's term.
    log_size: u64,
}

#[derive(Debug, Clone)]
enum Role {
    Candidate {
        term: Term,
        successes: HashMap<Address, u64>,
        outstanding_votes: HashSet<Address>,
        attempt_expiration: SystemTime,
    },
    Leader {
        term: Term,
        broadcast_indices: HashMap<Address, u64>,
    },
    Follower {
        leader: Address,
        leader_timeout: SystemTime,
        term: Term,
    },
}

#[derive(Debug, Clone)]
pub struct Replica<R: Rsm> {
    state: R,
    log: Vec<(Term, R::WriteReq)>,
    pending_requests: HashMap<usize, (Address, Option<u64>)>,
    committed_log_size: u64,
    io: Io,
    role: Role,
    peers: Vec<Address>,
    address: Address,
    leader_timeout: SystemTime,
}

impl<R: Rsm> Replica<R> {
    fn cron(&mut self) {
        let now = self.io.now();
        match &mut self.role {
            Role::Candidate {
                attempt_expiration, ..
            } => {
                if &now >= attempt_expiration {
                    self.become_candidate()
                }
            }
            Role::Follower { leader_timeout, .. } => {
                if &now >= leader_timeout {
                    self.become_candidate()
                }
            }
            Role::Leader {
                term,
                broadcast_indices,
            } => {
                // TODO handle timeouts

                for (peer, idx) in broadcast_indices {
                    let req = RsmMessage::AppendReq(AppendReq {
                        term: *term,
                        leader_committed_log_size: self.committed_log_size,
                        batch_start_log_index: *idx,
                        entries: self.log[*idx as usize..self.committed_log_size as usize]
                            .iter()
                            .cloned()
                            .collect(),
                        last_log_term: if *idx > 0 {
                            Some(self.log[(*idx - 1) as usize].0)
                        } else {
                            None
                        },
                    });

                    self.io.send(*peer, None, R::wrap(req));
                }
            }
        }
    }

    fn term(&self) -> Term {
        match &self.role {
            Role::Candidate { term, .. }
            | Role::Follower { term, .. }
            | Role::Leader { term, .. } => *term,
        }
    }

    fn become_candidate(&mut self) {
        let now = self.io.now();
        let expiration_ms = self.io.rand(150, 300);
        let term = self.term().increment();

        self.role = Role::Candidate {
            successes: HashMap::new(),
            attempt_expiration: now + Duration::from_millis(expiration_ms),
            outstanding_votes: self.peers().copied().collect(),
            term,
        };

        let vote_req = R::wrap(RsmMessage::<R>::VoteReq {
            proposed_leadership_term: term,
            last_log_term: if self.committed_log_size > 0 {
                self.log.get(self.committed_log_size as usize).map(|l| l.0)
            } else {
                None
            },
            committed_log_size: self.committed_log_size,
        });

        for peer in self.peers() {
            self.io.send(*peer, None, vote_req.clone());
        }
    }

    fn peers(&self) -> impl Iterator<Item = &Address> {
        self.peers.iter().filter(|p| *p != &self.address)
    }

    fn receive(&mut self, envelope: Envelope) {
        match R::unwrap(envelope.message) {
            Ok(RsmMessage::AppendReq(append_req)) => {
                self.handle_append_req(envelope.from, append_req)
            }
            Ok(RsmMessage::AppendRes(append_res)) => {
                self.handle_append_res(envelope.from, append_res)
            }
            Ok(RsmMessage::ReadReq(read_req)) => {
                self.handle_read_req(envelope.from, envelope.request_id, read_req)
            }
            Ok(RsmMessage::WriteReq(write_req)) => {
                self.handle_write_req(envelope.from, envelope.request_id, write_req)
            }
            Ok(RsmMessage::VoteReq {
                proposed_leadership_term,
                last_log_term,
                committed_log_size,
            }) => self.handle_vote_req(
                envelope.from,
                proposed_leadership_term,
                last_log_term,
                committed_log_size,
            ),
            Ok(RsmMessage::VoteRes {
                success,
                term,
                committed_log_size,
            }) => self.handle_vote_res(envelope.from, success, term, committed_log_size),
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

    fn handle_read_req(&mut self, from: Address, request_id: Option<u64>, req: R::ReadReq) {
        match &self.role {
            Role::Candidate { .. } => return,
            Role::Follower { leader, term, .. } => {
                let redirect_msg = R::wrap(RsmMessage::Redirect {
                    leader: *leader,
                    term: *term,
                });
                self.io.send(from, request_id, redirect_msg);

                return;
            }
            Role::Leader { term, .. } => {}
        }

        let res = self.state.read(req);

        let wrapped = R::wrap(RsmMessage::ReadRes(res));

        self.io.send(from, request_id, wrapped);
    }

    fn handle_write_req(&mut self, from: Address, request_id: Option<u64>, req: R::WriteReq) {
        match &self.role {
            Role::Candidate { .. } => return,
            Role::Follower { leader, term, .. } => {
                let redirect_msg = R::wrap(RsmMessage::Redirect {
                    leader: *leader,
                    term: *term,
                });
                self.io.send(from, request_id, redirect_msg);

                return;
            }
            Role::Leader { term, .. } => {
                self.log.push((*term, req));
                self.pending_requests
                    .insert(self.log.len(), (from, request_id));
            }
        }
    }

    fn handle_append_req(&mut self, from: Address, req: AppendReq<R::WriteReq>) {
        // if we don't follow them, possibly start doing so
        todo!()
    }

    fn handle_append_res(&mut self, from: Address, req: AppendRes) {
        todo!()
    }

    fn handle_vote_req(
        &mut self,
        from: Address,
        proposed_leadership_term: Term,
        last_log_term: Option<Term>,
        committed_log_size: u64,
    ) {
        if proposed_leadership_term > self.term() && committed_log_size >= self.committed_log_size {
            self.io.send(
                from,
                None,
                R::wrap(RsmMessage::<R>::VoteRes {
                    success: true,
                    term: self.term(),
                    committed_log_size: self.committed_log_size,
                }),
            );
            let now = self.io.now();
            let expiration_ms = self.io.rand(150, 300);
            self.role = Role::Follower {
                leader: from,
                term: proposed_leadership_term,
                leader_timeout: now + Duration::from_millis(expiration_ms),
            };
        } else {
            self.io.send(
                from,
                None,
                R::wrap(RsmMessage::<R>::VoteRes {
                    success: false,
                    term: self.term(),
                    committed_log_size: self.committed_log_size,
                }),
            );
        }
    }

    fn handle_vote_res(
        &mut self,
        from: Address,
        success: bool,
        accepted_term: Term,
        committed_log_size: u64,
    ) {
        if !success {
            return;
        }

        if let Role::Candidate {
            term,
            outstanding_votes,
            successes,
            ..
        } = &mut self.role
        {
            if *term != accepted_term {
                return;
            }

            if !outstanding_votes.remove(&from) {
                return;
            }

            successes.insert(from, committed_log_size);

            if successes.len() >= outstanding_votes.len() {
                self.role = Role::Leader {
                    term: *term,
                    broadcast_indices: std::mem::take(successes),
                };
            }
        }
    }
}
