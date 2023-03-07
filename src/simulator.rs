use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
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
    // keyed by timeout and unique ID
    timers: BTreeMap<(u64, u64), OneshotSender<()>>,
    // receiver address -> (timeout, oneshot)
    receivers: BTreeMap<Address, (u64, OneshotSender<Envelope>)>,
    // (address, request id) -> (timeout, oneshot)
    requests: BTreeMap<(Address, u64), (u64, OneshotSender<Envelope>)>,
    can_receive: BTreeMap<Address, Vec<Envelope>>,
}

#[derive(Debug, Default)]
pub struct Simulator {
    mu: Arc<Mutex<SimulatorState>>,
    cv: Arc<Condvar>,
    idgen: Arc<AtomicU64>,
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

        drop(sim);

        self.cv.notify_all();
    }

    fn receive(&self, timeout: Duration, receiver_address: &Address) -> ReceiveFuture {
        let (sender, receiver) = channel();

        let mut sim = self.mu.lock().unwrap();

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
}
