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
use std::io;
use std::process::Command;
use std::path::Path;
use std::ffi::{OsString};

#[derive(Debug, StructOpt)]
struct Opts {
    /// Path to disk image
    disk: String,

    /// Name of VM
    #[structopt(long = "name")]
    name: Option<String>,

    /// Libvirt storage pool
    #[structopt(long = "pool", default_value = "default")]
    pool: String,

    /// The libvirt URI
    #[structopt(short = "c")]
    uri: Option<String>,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

pub(crate) trait CommandRunExt {
    fn run(&mut self) -> io::Result<()>;
}

impl CommandRunExt for Command {
    fn run(&mut self) -> io::Result<()> {
        let r = self.status()?;
        if !r.success() {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Child [{:?}] exited: {}", self, r)));
        }
        Ok(())
    }
}

fn virsh(opts: &Opts) -> Command
{
    let mut cmd = Command::new("virsh");
    if let Some(ref uri) = opts.uri.as_ref() {
        cmd.arg("--connect");
        cmd.arg(uri);
    }
    cmd
}

fn main() -> CliResult {
    let opts = Opts::from_args();
    opts.verbosity.setup_env_logger("head")?;

    let name = opts.name.as_ref().map(|s| s.to_string()).unwrap_or_else(|| {
        Path::new(opts.disk.as_str()).file_stem().expect("disk name").to_str().unwrap().to_string()
    });

    let size = std::fs::metadata(opts.disk.as_str())?.len();

    //virsh(&opts).args(&["vol-create-as", "--pool"]).arg(opts.pool.as_str()).arg(opts.disk.as_str()).run()?;
    Ok(())
}
