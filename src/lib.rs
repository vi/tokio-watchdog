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
    /// the main thing
    del : Option<Delay>,
    /// saved duration for petting and rearming
    dur : Duration,
    /// used to track pets when `del` is None (unarmed)
    ins : Instant,
}

type H = Arc<Mutex<Impl>>;
//type H = Rc<RefCell<Impl>>>;

pub struct Watchdog(H);

impl Watchdog {
    pub fn new(dur: Duration) -> Self {
        let ins = Instant::now() + dur;
        let del = Delay::new(ins);
        let i = Impl { del:Some(del), dur, ins };
        Watchdog(Arc::new(Mutex::new(i)))
    }
    /// Get the duration. Returns 0 on internal error.
    pub fn duration(&self) -> Duration {
        if let Ok(mut g) = self.0.lock() {
            g.dur
        } else {
            Duration::from_secs(0)
        }
    }
    /// Set new duration, also adjusting the timer state
    pub fn set_duration(&mut self, dur: Duration) {
        if let Ok(mut g) = self.0.lock() {
            g.ins = g.ins - g.dur + dur;
            g.dur = dur;
            let i = g.ins;
            if let Some(ref mut d) = g.del {
                d.reset(i);
            }
        }
    }
    pub fn handle(&self) -> Pet {
        self.into()
    }
}

/// Reset/restart the watchdog, so it don't activate
#[derive(Clone)]
pub struct Pet(H);
impl Pet {
    /// Reset/restart the watchdog, so it don't activate
    ///
    /// Call it periodically from various places
    pub fn pet(&self) {
        if let Ok(mut g) = self.0.lock() {
            let i = Instant::now() + g.dur;
            g.ins = i;
            if let Some(ref mut x) = g.del {
                x.reset(i);
            }
        } else {
            // don't know what to do here
            // XXX
        }
    }
    
    /// Get how much time remains before the watchdog activates
    ///
    /// None means it is already active
    ///
    /// Some(0) is returned on internal error
    pub fn get_remaining_time(&self) -> Option<Duration> {
        if let Ok(g) = self.0.lock() {
            let now = Instant::now();
            let i = g.ins;
            if now > i {
                None
            } else {
                Some(i - now)
            }
        } else {
            Some(Duration::from_secs(0))
        }
    }
}

impl<'a> From<&'a Watchdog> for Pet {
    fn from(w : &'a Watchdog) -> Pet {
        Pet(w.0.clone())
    }
}

/// Result returned from a fired Watchdog.
/// Can be used to rewind (activate again) watchdog, preserving `Pet` handles pointing to it.
pub struct Rearm(H);
impl Rearm {
    pub fn rearm(self) -> Watchdog {
        Watchdog(self.0)
    }
    
    pub fn rearm_with_duration(self, dur: Duration) -> Watchdog {
        let mut w = Watchdog(self.0);
        w.set_duration(dur);
        w
    }
}

impl Future for Watchdog {
    type Item = Rearm;
    /// at_capacity error may also be returned on internal Mutex problems
    type Error = tokio_timer::Error;
    
    fn poll(&mut self) -> futures::Poll<Rearm, tokio_timer::Error> {
        if let Ok(mut g) = self.0.lock() {
            if let Some(ref mut d) = g.del {
                match d.poll() {
                    Ok(Async::Ready(())) => Ok(Async::Ready(
                        Rearm(self.0.clone())
                    )),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Err(x) => Err(x),
                }
            } else {
                Ok(Async::Ready(Rearm(self.0.clone())))
            }
        } else {
            // unlikely to happen, just some filler
            Err(tokio_timer::Error::at_capacity())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
