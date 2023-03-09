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

use std::collections::BTreeMap;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use futures::executor::block_on;

use crate::*;

pub struct MachineManager {
    coordinator: Option<Replica<Coordinator>>,
    rsms: BTreeMap<RsmId, Replica<Shard>>,
    io: Io,
}

impl MachineManager {
    pub fn recover<P: AsRef<Path>>(path: P) -> io::Result<MachineManager> {
        todo!()
    }

    pub fn run(&mut self) {
        while !self.io.should_shutdown() {
            if let Ok(envelope) = block_on(self.io.receive()) {
                self.handle(envelope);
            } else {
                self.cron();
            }
        }
    }

    fn handle(&mut self, envelope: Envelope) {
        use common::Message::*;
        match envelope.message {
            _ => todo!(),
        }
    }

    fn cron(&mut self) {
        todo!()
    }
}
