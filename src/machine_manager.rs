use std::collections::BTreeMap;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

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
            if let Ok(envelope) = self.io.receive() {
                self.handle(envelope);
            } else {
                self.cron();
            }
        }
    }

    fn handle(&mut self, envelope: Envelope) {
        use common::Message::*;
        match envelope.message {
            CoordinatorReq(req) => {
                if let Some(ref coordinator) = self.coordinator {
                    coordinator.handle(envelope)
                }
            }
            ShardReq(req) => {}
            Response(response) => {
                panic!("Io::request returned unexpected Response: {:?}", response);
            }
        }
    }

    fn cron(&mut self) {
        todo!()
    }
}
