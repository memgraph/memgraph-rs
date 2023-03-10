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

use std::collections::{BTreeMap, HashSet};
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use bincode::{deserialize, serialize};
use futures::executor::block_on;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::*;

const META_KEY: &[u8] = b"__machine_metadata";
const COORDINATOR_KEY: &[u8] = b"__coordinator";

fn rsm_id_to_serialized_name(id: RsmId) -> Vec<u8> {
    format!("__rsm_{id}").into_bytes()
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct MachineMetadata {
    rsms: HashSet<RsmId>,
}

pub struct MachineManager {
    coordinator: Option<Replica<Coordinator>>,
    rsms: BTreeMap<RsmId, Replica<Shard>>,
    io: Io,
    db: sled::Db,
}

impl MachineManager {
    pub fn recover<P: AsRef<Path>>(path: P, io: Io) -> io::Result<MachineManager> {
        let db = match sled::open(path) {
            Ok(db) => db,
            Err(sled::Error::Io(io)) => return Err(io),
            Err(other) => panic!(
                "unexpected error when opening MachineManager state: {:?}",
                other
            ),
        };

        let metadata: MachineMetadata = db
            .get(META_KEY)
            .unwrap()
            .map(|bytes| deserialize(&bytes).unwrap())
            .unwrap_or_default();

        let rsms_res: io::Result<BTreeMap<RsmId, Replica<Shard>>> = metadata
            .rsms
            .iter()
            .map(|id| {
                let tree_name = rsm_id_to_serialized_name(*id);
                let tree = db.open_tree(tree_name).unwrap();
                let rsm: Replica<Shard> = Replica::recover(tree, io.clone())?;
                Ok((*id, rsm))
            })
            .collect();

        let rsms = rsms_res?;

        let rsm_db = db.open_tree(COORDINATOR_KEY).unwrap();
        let coordinator = Some(Replica::recover(rsm_db, io.clone())?);

        Ok(MachineManager {
            rsms,
            db,
            coordinator,
            io,
        })
    }

    pub fn run(&mut self) {
        while !self.io.should_shutdown() {
            if let Ok(envelope) = block_on(self.io.receive()) {
                self.handle(envelope);
            } else {
                self.cron();
            };
        }
    }

    fn handle(&mut self, envelope: Envelope) {
        use common::Message::*;
        match &envelope.message {
            Message::Shard(_) => {
                if let Some(rsm) = self.rsms.get_mut(&envelope.to.id) {
                    rsm.receive(envelope);
                }
            }
            Message::Coordinator(_) => {
                if let Some(coordinator) = &mut self.coordinator {
                    coordinator.receive(envelope);
                }
            }
        }
    }

    fn cron(&mut self) {
        if let Some(coordinator) = &mut self.coordinator {
            coordinator.cron();
        }

        for (_, rsm) in &mut self.rsms {
            rsm.cron();
        }
    }
}
