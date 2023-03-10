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
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Condvar, Mutex,
};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};

use futures::channel::oneshot::{channel, Receiver as OneshotReceiver, Sender as OneshotSender};
use rand::{Rng, SeedableRng};

use crate::*;

#[derive(Debug)]
struct SimulatorState {
    should_shutdown: bool,
    now_us: u64,
    // keyed by timeout and unique ID
    timers: BTreeMap<(u64, u64), OneshotSender<()>>,
    // receiver address -> (timeout, oneshot)
    receivers: BTreeMap<Address, (u64, OneshotSender<Envelope>)>,
    // (address, request id) -> (timeout, oneshot)
    requests: BTreeMap<(Address, u64), (u64, OneshotSender<Envelope>)>,
    can_receive: BTreeMap<Address, Vec<Envelope>>,
    rng: rand_chacha::ChaCha8Rng,
    server_count: usize,
}

impl SimulatorState {
    fn maybe_tick(&mut self) {
        if !self.quiescent() {
            return;
        }

        self.now_us += 1000;

        self.receivers
            .retain(|_addr, (timeout, _oneshot)| *timeout > self.now_us);

        self.requests
            .retain(|(_addr, _request_id), (timeout, _oneshot)| *timeout > self.now_us);
    }

    fn quiescent(&self) -> bool {
        let n_quiescent = self.receivers.len();
        n_quiescent == self.server_count
    }
}

impl Default for SimulatorState {
    fn default() -> SimulatorState {
        SimulatorState {
            should_shutdown: Default::default(),
            now_us: Default::default(),
            timers: Default::default(),
            receivers: Default::default(),
            requests: Default::default(),
            can_receive: Default::default(),
            rng: rand_chacha::ChaCha8Rng::from_seed([0; 32]),
            server_count: 1,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Simulator {
    mu: Arc<Mutex<SimulatorState>>,
    cv: Arc<Condvar>,
    idgen: Arc<AtomicU64>,
}

impl Simulator {
    pub fn start_ticker_thread(&self) -> std::thread::JoinHandle<()> {
        let sim = self.clone();

        std::thread::spawn(move || sim.ticker())
    }

    fn ticker(self) {
        loop {
            let mut sim = self.mu.lock().unwrap();
            sim.maybe_tick();
            if sim.should_shutdown {
                return;
            }
        }
    }
}

impl Handle for Simulator {
    fn shutdown(&self) {
        self.mu.lock().unwrap().should_shutdown = true;
        self.cv.notify_all();
    }

    fn should_shutdown(&self) -> bool {
        self.mu.lock().unwrap().should_shutdown
    }

    fn send(&self, envelope: Envelope) {
        let mut sim = self.mu.lock().unwrap();
        self.cv.notify_all();

        // see if there is a request that corresponds to a request_id and recipient
        if let Some(request_id) = envelope.request_id {
            if let Some((_timeout, sender)) = sim.requests.remove(&(envelope.to, request_id)) {
                sender.send(envelope);
                return;
            }
        }

        // see if there is a receiver who can accept our message
        let rx_opt = sim.receivers.remove(&envelope.to);
        if let Some((_timeout, sender)) = rx_opt {
            sender.send(envelope);
            return;
        }

        // otherwise, add it to can_receive
        let mut entry = sim.can_receive.entry(envelope.to).or_default();

        entry.push(envelope);
    }

    fn receive(&self, timeout: Duration, receiver_address: &Address) -> ReceiveFuture {
        let (sender, receiver) = channel();

        let mut sim = self.mu.lock().unwrap();
        self.cv.notify_all();

        if let Some(ref mut receivable) = sim.can_receive.get_mut(receiver_address) {
            if !receivable.is_empty() {
                // can immediately fill the oneshot that we return
                sender.send(receivable.pop().unwrap());
                return ReceiveFuture { inner: receiver };
            }
        }

        let timeout = u64::try_from(timeout.as_micros()).unwrap() + sim.now_us;
        sim.receivers.insert(*receiver_address, (timeout, sender));

        ReceiveFuture { inner: receiver }
    }

    fn now(&self) -> SystemTime {
        let now_us = self.mu.lock().unwrap().now_us;

        SystemTime::UNIX_EPOCH + Duration::from_micros(now_us)
    }

    fn timer(&self, duration: Duration) -> TimerFuture {
        let (sender, receiver) = channel();
        let unique_id = self.idgen();

        let mut sim = self.mu.lock().unwrap();
        let timeout = u64::try_from(duration.as_micros()).unwrap() + sim.now_us;
        sim.timers.insert((timeout, unique_id), sender);

        TimerFuture { inner: receiver }
    }

    fn request(&self, timeout: Duration, request_envelope: Envelope) -> RequestFuture {
        let (sender, receiver) = channel();
        let ret = RequestFuture { inner: receiver };

        let mut sim = self.mu.lock().unwrap();
        self.cv.notify_all();

        // register our pending request
        let timeout = u64::try_from(timeout.as_micros()).unwrap() + sim.now_us;
        let last = sim.requests.insert(
            (request_envelope.from, request_envelope.request_id.unwrap()),
            (timeout, sender),
        );
        assert!(last.is_none());

        // see if we can immediately deliver the request to the receiver
        let rx_opt = sim.receivers.remove(&request_envelope.to);
        if let Some((_timeout, rx_sender)) = rx_opt {
            rx_sender.send(request_envelope);
            return ret;
        }

        // otherwise, place the request as a receivable message for the destination
        let mut entry = sim.can_receive.entry(request_envelope.to).or_default();
        entry.push(request_envelope);

        ret
    }

    fn idgen(&self) -> u64 {
        self.idgen.fetch_add(1, Ordering::Relaxed)
    }

    fn rand(&self, low: u64, high: u64) -> u64 {
        let mut sim = self.mu.lock().unwrap();
        sim.rng.gen_range(low..high)
    }
}
