#![allow(dead_code,unused)]

extern crate tokio_timer;
extern crate futures;


use tokio_timer::Delay;
use futures::Future;

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

impl Future for Watchdog {
    type Item = ();
    type Error = tokio_timer::Error;
    
    fn poll(&mut self) -> futures::Poll<(), tokio_timer::Error> {
        self.del.poll()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
