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

pub trait Rsm: Sized + Serialize + DeserializeOwned {
    type ReadReq: Serialize + DeserializeOwned;
    type ReadRes: Serialize + DeserializeOwned;
    type WriteReq: Serialize + DeserializeOwned;
    type WriteRes: Serialize + DeserializeOwned;

    fn read(&self, request: Self::ReadReq) -> Self::ReadRes;
    fn write(&mut self, request: Self::WriteReq) -> Self::WriteRes;
    fn recover<P: AsRef<Path>>(path: P) -> io::Result<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Term(u64);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VoteReq {
    term: Term,
    log_size: u64,
    last_log_term: Term,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VoteRes {
    success: bool,
    term: Term,
    log_size: u64,
    last_log_term: Term,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteReq<R: Rsm> {
    req: R::WriteReq,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteRes<R: Rsm> {
    res: R::WriteRes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadReq<R: Rsm> {
    req: R::ReadReq,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadRes<R: Rsm> {
    res: R::ReadRes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendReq<R: Rsm> {
    pd: PhantomData<R>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppendRes<R: Rsm> {
    pd: PhantomData<R>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Replica<R: Rsm> {
    state: R,
}
