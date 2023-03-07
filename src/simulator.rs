use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, SystemTime};

use crate::*;

#[derive(Debug, Default)]
struct SimulatorRequestSharedState {
    waker: Option<Waker>,
}

struct SimulatorFuture {}

struct SimulatorPromise {}

#[derive(Debug, Default)]
struct SimulatorState {
    should_shutdown: bool,
    now_us: u64,
}

#[derive(Debug, Default)]
pub struct Simulator {
    mu: Arc<Mutex<SimulatorState>>,
    cv: Arc<Condvar>,
}

impl Handle for Simulator {
    fn should_shutdown(&self) -> bool {
        self.mu.lock().unwrap().should_shutdown
    }

    fn send(&self, envelope: Envelope) {
        todo!()
    }

    fn receive(&self, timeout: Duration) -> ReceiveFuture {
        todo!()
    }

    fn now(&self) -> SystemTime {
        let now_us = self.mu.lock().unwrap().now_us;

        SystemTime::UNIX_EPOCH + Duration::from_micros(now_us)
    }

    fn timer(&self, duration: Duration) -> TimerFuture {
        todo!()
    }
}
