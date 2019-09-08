#!/usr/bin/env run-cargo-script
//! https://timryan.org/2018/07/02/moving-from-the-shell-to-rust-with-commandspec.html
//!
//! ```cargo
//! [package]
//! edition = "2018"
//! 
//! [dependencies]
//! quicli = "0.4"
//! structopt = "0.2"
//! ```
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// Path to disk image
    disk: String,

    /// The libvirt URI
    #[structopt(short = "c")]
    uri: Option<String>,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Opts::from_args();
    args.verbosity.setup_env_logger("head")?;

    dbg!(&args.disk);
    dbg!(&args.uri);
    Ok(())
}
