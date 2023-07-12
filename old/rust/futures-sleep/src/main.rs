extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate clap;

use std::{io,thread};
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use futures::prelude::*;
use futures::{Future,future,Stream};
use futures::stream::FuturesUnordered;
use futures_cpupool::CpuPool;

fn run(n: u32) -> io::Result<()> {
    let pool : CpuPool = CpuPool::new_num_cpus();
    let mut rxs = FuturesUnordered::new();

    for _ in 0..n {
        let f = pool.spawn_fn(|| {
            thread::sleep(Duration::from_millis(100));
            Ok::<(), ()>(())
        });
        rxs.push(f);
    }

    // Stole this from the futures_unordered.rs test oneshots()
    // This incantation is apparently the way to synchronously
    // wait on the FuturesUnordered set.
    future::lazy(move || {
        loop {
            if let Ok(Async::Ready(None)) = rxs.poll() {
                return Ok::<(), ()>(());
            }
        }
    }).wait().unwrap();
    Ok(())
}

fn main() {
    let matches = App::new("futures-sleep")
        .version("0.1")
        .about("Benchmark futures")
        .arg(Arg::with_name("num").required(true))
        .get_matches();
    let n = u32::from_str(matches.value_of("num").unwrap()).unwrap();

    match run(n) {
        Ok(_) => { },
        Err(e) => { eprintln!("{:?}", e);
                    std::process::exit(1) }
    }
}
