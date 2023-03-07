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
    pub inner: futures::channel::oneshot::Receiver<()>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pin = unsafe { std::pin::Pin::new_unchecked(&mut self.inner) };

        pin.as_mut().poll(cx).map(|_| ())
    }
}

pub struct ReceiveFuture {
    pub inner: futures::channel::oneshot::Receiver<Envelope>,
}

impl Future for ReceiveFuture {
    type Output = io::Result<Envelope>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pin = unsafe { std::pin::Pin::new_unchecked(&mut self.inner) };

        match pin.as_mut().poll(cx) {
            Poll::Ready(Ok(envelope)) => Poll::Ready(Ok(envelope)),
            Poll::Ready(Err(_)) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "request timed out",
            ))),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub trait Handle: std::fmt::Debug {
    fn shutdown(&self);

    fn should_shutdown(&self) -> bool;

    fn send(&self, envelope: Envelope);

    fn receive(&self, timeout: Duration, receiver_address: &Address) -> ReceiveFuture;

    fn now(&self) -> SystemTime;

    fn timer(&self, duration: Duration) -> TimerFuture;
}

#[derive(Debug, Clone)]
pub struct Io {
    pub address: Address,
    pub timeout: Duration,
    pub handle: Arc<dyn Handle>,
}

impl Io {
    pub fn send(&self, envelope: Envelope) {
        self.handle.send(envelope)
    }

    pub async fn receive(&self) -> io::Result<Envelope> {
        self.handle.receive(self.timeout, &self.address).await
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
