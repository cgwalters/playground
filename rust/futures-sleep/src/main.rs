extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate clap;

use std::{io,thread};
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use futures::prelude::*;
use futures::{Future,future,Stream,stream};
use futures_cpupool::CpuPool;

fn run(n: u32) -> io::Result<()> {
    let pool : CpuPool = CpuPool::new_num_cpus();

    let all = stream::iter_ok(0..n).map(|i| {
        pool.spawn_fn(|| {
            Ok(thread::sleep(Duration::from_millis(100)))
        })
    });

    all.fold((), |acc, x| { future::ok(acc) }).wait()
}

fn main() {
    let matches = App::new("futures-sleep")
        .version("0.1")
        .about("Benchmark futures")
        .arg(Arg::with_name("num").required(true))
        .get_matches();
    let n = u32::from_str(matches.value_of("num").unwrap()).unwrap();

    std::process::exit(
        match run(n) {
            Ok(()) => 0,
            Err(e) => { println!("{:?}", e); 1 }
        }
    )
}
