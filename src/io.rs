use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;

use crate::*;

#[derive(Debug, Clone)]
pub struct Io {
    pub timeout: Duration,
    pub should_shutdown: Arc<AtomicBool>,
}

impl Io {
    pub fn receive(&mut self) -> Result<Envelope, Timeout> {
        todo!()
    }

    pub fn should_shutdown(&self) -> bool {
        todo!()
    }
}
