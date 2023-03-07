use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime};

use crate::*;

pub struct TimerFuture {
    inner: Box<dyn Future<Output = ()>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pin = unsafe { std::pin::Pin::new_unchecked(&mut *self.inner) };
        pin.as_mut().poll(cx)
    }
}

pub struct ReceiveFuture {
    inner: Box<dyn Future<Output = io::Result<Envelope>>>,
}

impl Future for ReceiveFuture {
    type Output = io::Result<Envelope>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pin = unsafe { std::pin::Pin::new_unchecked(&mut *self.inner) };
        pin.as_mut().poll(cx)
    }
}

pub trait Handle: std::fmt::Debug {
    fn should_shutdown(&self) -> bool;

    fn send(&self, envelope: Envelope);

    fn receive(&self, timeout: Duration) -> ReceiveFuture;

    fn now(&self) -> SystemTime;

    fn timer(&self, duration: Duration) -> TimerFuture;
}

#[derive(Debug, Clone)]
pub struct Io {
    pub timeout: Duration,
    pub handle: Arc<dyn Handle>,
}

impl Io {
    pub fn send(&self, envelope: Envelope) {
        self.handle.send(envelope)
    }

    pub async fn receive(&self) -> io::Result<Envelope> {
        self.handle.receive(self.timeout).await
    }

    pub fn should_shutdown(&self) -> bool {
        self.handle.should_shutdown()
    }

    pub fn now(&self) -> SystemTime {
        self.handle.now()
    }

    pub fn timer(&self, duration: Duration) -> TimerFuture {
        self.handle.timer(duration)
    }
}
