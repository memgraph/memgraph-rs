use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::{atomic::AtomicBool, Arc};
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime};

use crate::*;

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
}

#[derive(Debug, Clone)]
pub struct Io {
    pub timeout: Duration,
    pub should_shutdown: Arc<AtomicBool>,
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
        todo!()
    }

    pub fn now(&self) -> SystemTime {
        self.handle.now()
    }
}
