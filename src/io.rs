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

pub struct RequestFuture {
    pub inner: futures::channel::oneshot::Receiver<Envelope>,
}

impl Future for RequestFuture {
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

pub trait Handle: std::fmt::Debug + Send + Sync {
    fn shutdown(&self);

    fn should_shutdown(&self) -> bool;

    fn send(&self, envelope: Envelope);

    fn receive(&self, timeout: Duration, receiver_address: &Address) -> ReceiveFuture;

    fn request(&self, timeout: Duration, request_envelope: Envelope) -> RequestFuture;

    fn idgen(&self) -> u64;

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
    pub fn send(&self, to: Address, request_id: Option<u64>, message: Message) {
        let envelope = Envelope {
            to: to,
            from: self.address,
            request_id,
            message,
        };
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

    pub fn request(&self, to: Address, request: Message) -> RequestFuture {
        let request_id = Some(self.handle.idgen());
        let request_envelope = Envelope {
            from: self.address,
            to,
            message: request,
            request_id,
        };
        self.handle.request(self.timeout, request_envelope)
    }
}
