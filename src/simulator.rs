use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};

use futures::channel::oneshot::{channel, Receiver as OneshotReceiver, Sender as OneshotSender};

use crate::*;

#[derive(Debug, Default)]
struct SimulatorState {
    should_shutdown: bool,
    now_us: u64,
    unique_id_generator: u64,
    // keyed by timeout and unique ID
    timers: BTreeMap<(u64, u64), OneshotSender<()>>,
    // keyed by receiver Address and timeout
    receivers: BTreeMap<Address, (u64, OneshotSender<Envelope>)>,
    can_receive: BTreeMap<Address, Vec<Envelope>>,
}

impl SimulatorState {
    fn idgen(&mut self) -> u64 {
        self.unique_id_generator += 1;
        self.unique_id_generator
    }
}

#[derive(Debug, Default)]
pub struct Simulator {
    mu: Arc<Mutex<SimulatorState>>,
    cv: Arc<Condvar>,
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

        // see if there is a receiver who can accept our message
        let rx_opt = sim.receivers.remove(&envelope.to);
        if let Some((_timeout, sender)) = rx_opt {
            sender.send(envelope);
            return;
        }

        // otherwise, add it to can_receive
        let mut entry = sim.can_receive.entry(envelope.to).or_default();

        entry.push(envelope);

        drop(sim);

        self.cv.notify_all();
    }

    fn receive(&self, timeout: Duration, receiver_address: &Address) -> ReceiveFuture {
        let mut sim = self.mu.lock().unwrap();

        let (sender, receiver) = channel();

        if let Some(ref mut receivable) = sim.can_receive.get_mut(receiver_address) {
            if !receivable.is_empty() {
                // can immediately fill the oneshot that we return
                sender.send(receivable.pop().unwrap());
                return ReceiveFuture { inner: receiver };
            }
        }

        let timeout = u64::try_from(timeout.as_micros()).unwrap() + sim.now_us;
        sim.receivers.insert(*receiver_address, (timeout, sender));

        drop(sim);

        self.cv.notify_all();

        ReceiveFuture { inner: receiver }
    }

    fn now(&self) -> SystemTime {
        let now_us = self.mu.lock().unwrap().now_us;

        SystemTime::UNIX_EPOCH + Duration::from_micros(now_us)
    }

    fn timer(&self, duration: Duration) -> TimerFuture {
        let mut sim = self.mu.lock().unwrap();
        let timeout = u64::try_from(duration.as_micros()).unwrap() + sim.now_us;
        let unique_id = sim.idgen();
        let (sender, receiver) = channel();
        sim.timers.insert((timeout, unique_id), sender);

        TimerFuture { inner: receiver }
    }
}
