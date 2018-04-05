#![allow(dead_code,unused)]

extern crate tokio_timer;
extern crate futures;


use tokio_timer::Delay;
use futures::Future;
use futures::Async;

use std::time::{Duration, Instant};


pub struct Watchdog {
    del : Delay,
    dur : Duration,
}

/// Reset/restart the watchdog, so it don't activate
pub struct Pet {
    // TODO
}
impl Pet {
    /// Reset/restart the watchdog, so it don't activate
    pub fn pet(&self) {
        // TODO
    }
}

impl Watchdog {
    fn new(dur: Duration) -> Self {
        Watchdog {
            del: Delay::new(Instant::now() + dur),
            dur: dur,
        }
    }
}

/// Result returned from a fired Watchdog.
/// Can be used to rewind watchdog, preserving `Pet` handles pointing to it.
pub struct Rewind {
}
impl Rewind {
    fn rewind(dur: Duration) -> Watchdog {
        unimplemented!()
    }
}

impl Future for Watchdog {
    type Item = Rewind;
    type Error = tokio_timer::Error;
    
    fn poll(&mut self) -> futures::Poll<Rewind, tokio_timer::Error> {
        match self.del.poll() {
            Ok(Async::Ready(())) => Ok(Async::Ready(unimplemented!())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(x) => Err(x),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
