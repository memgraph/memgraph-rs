use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::{atomic::AtomicBool, Arc};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use crate::*;

pub struct ReceiveFuture {
    inner: Pin<Box<dyn Future<Output = io::Result<Envelope>>>>,
}

impl Future for ReceiveFuture {
    type Output = io::Result<Envelope>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

pub trait Handle: std::fmt::Debug {
    fn should_shutdown(&self) -> bool;

    fn send(&self, envelope: Envelope);

    fn receive(&self, timeout: Duration) -> ReceiveFuture;
}

#[derive(Debug, Clone)]
pub struct Io {
    pub timeout: Duration,
    pub should_shutdown: Arc<AtomicBool>,
    pub handle: Arc<dyn Handle>,
}

impl Io {
    pub fn receive(&mut self) -> Result<Envelope, Timeout> {
        todo!()
    }

    pub fn should_shutdown(&self) -> bool {
        todo!()
    }
}
