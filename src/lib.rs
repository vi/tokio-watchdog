#![allow(dead_code,unused)]

extern crate tokio_timer;
extern crate futures;


use tokio_timer::Delay;
use futures::Future;
use futures::Async;

use std::sync::Arc;
use std::sync::Mutex;

use std::time::{Duration, Instant};


// Or Rc<RefCell<>>, selected by cargo features?
type H = Arc<Mutex<Option<Delay>>>;

pub struct Watchdog {
    del : H,
    dur : Duration,
}

impl Watchdog {
    fn new(dur: Duration) -> Self {
        let d = Delay::new(Instant::now() + dur);
        Watchdog {
            del: Arc::new(Mutex::new(Some(d))),
            dur: dur,
        }
    }
    fn handle(&self) -> Pet {
        self.into()
    }
}

/// Reset/restart the watchdog, so it don't activate
pub struct Pet {
    del : H,
}
impl Pet {
    /// Reset/restart the watchdog, so it don't activate
    pub fn pet(&self) {
        let mut g = self.del.lock().unwrap();
        if let Some(ref mut x) = *g {
            x.reset(Instant::now() /* + ?  XXX */);
        }
    }
}

impl<'a> From<&'a Watchdog> for Pet {
    fn from(w : &'a Watchdog) -> Pet {
        Pet {
            del: w.del.clone(),
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
        let mut g = self.del.lock().unwrap();
        match g.poll() {
            Ok(Async::Ready(Some(()))) => Ok(Async::Ready(unimplemented!())),
            Ok(Async::Ready(None)) => Ok(Async::Ready(unimplemented!())),
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
