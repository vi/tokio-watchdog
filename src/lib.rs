#![allow(dead_code,unused)]

extern crate tokio_timer;
extern crate futures;


use tokio_timer::Delay;
use futures::Future;
use futures::Async;

use std::sync::Arc;
use std::sync::Mutex;

use std::rc::Rc;
use std::cell::RefCell;

use std::time::{Duration, Instant};

struct Impl {
    del : Option<Delay>,
    dur : Duration,
}

type H = Arc<Mutex<Impl>>;
//type H = Rc<RefCell<Option<Delay>>>;

pub struct Watchdog(H);

impl Watchdog {
    fn new(dur: Duration) -> Self {
        let del = Delay::new(Instant::now() + dur);
        let i = Impl { del:Some(del), dur };
        Watchdog(Arc::new(Mutex::new(i)))
    }
    fn handle(&self) -> Pet {
        self.into()
    }
}

/// Reset/restart the watchdog, so it don't activate
pub struct Pet(H);
impl Pet {
    /// Reset/restart the watchdog, so it don't activate
    pub fn pet(&self) {
        let mut g = self.0.lock().unwrap(); // XXX
        let d = g.dur;
        if let Some(ref mut x) = g.del {
            x.reset(Instant::now() + d);
        }
    }
}

impl<'a> From<&'a Watchdog> for Pet {
    fn from(w : &'a Watchdog) -> Pet {
        Pet(w.0.clone())
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
        let mut g = self.0.lock().unwrap();
        if let Some(ref mut d) = g.del {
            match d.poll() {
                Ok(Async::Ready(())) => Ok(Async::Ready(unimplemented!())),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(x) => Err(x),
            }
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
