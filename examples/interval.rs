extern crate tokio;
extern crate tokio_watchdog;

use tokio::runtime;
use tokio::prelude::{Future, Stream, future};
use tokio::timer::Interval;

use std::time::{Instant, Duration};

fn main() {
    
    let mut r = runtime::Builder::new().build().unwrap();
    
    let watch = tokio_watchdog::Watchdog::new(Duration::from_secs(1));
    
    let p = watch.handle();
    
    let task1 = watch.and_then(|_| {
        println!("BLAM!");
        ::std::process::exit(0);
        #[allow(unreachable_code)]
        future::ok(())
    });
    
    
    
    let intvl = Interval::new(Instant::now(), Duration::new(0, 100_000_000));
    let lazy_time = Instant::now() + Duration::from_secs(2);
    let task2 = intvl.for_each(move |i| {
        if i < lazy_time {
            println!("pet");
            p.pet();
        } else {
            println!("no pet");
        }
        future::ok(())
    });
    
    r.spawn(task1.map_err(|_|()));
    r.spawn(task2.map_err(|_|()));
    
    r.shutdown_on_idle().wait().unwrap();
}
