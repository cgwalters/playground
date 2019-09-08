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
//! duct = "0.12.0"
//! ```
use quicli::prelude::*;
use structopt::StructOpt;
use duct::cmd;
use std::ffi::{OsString};

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

fn virsh<U, V>(opts: &Opts, args: U) -> duct::Expression
where
    U: IntoIterator<Item = V>,
    V: Into<OsString>,
{
    let mut fullargs : Vec<OsString> = Vec::new();
    if let Some(ref uri) = opts.uri.as_ref() {
        fullargs.extend(["--connect", uri].into_iter().map(|s|s.into()));
    }
    fullargs.extend(args.into_iter().map(|s| s.into()));
    cmd("virsh", fullargs)
}

fn main() -> CliResult {
    let opts = Opts::from_args();
    opts.verbosity.setup_env_logger("head")?;

    virsh(&opts, &[opts.disk.as_str()]).run()?;
    Ok(())
}
