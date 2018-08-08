extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use std::process::{Command, Stdio};
use failure::Error;
use clap::{App, Arg};

struct ImageStreamTagItem {
    created: String,
    image: String,
}

struct ImageStreamTag {
    tag: String,
    items: Vec<ImageStreamTagItem>
}

struct ImageStreamStatus {
    tags: Vec<ImageStreamTag>,
}

struct ImageStream {
    status: ImageStreamStatus,
}

fn run(imagestream: &str) -> Result<(), Error> {
    let oc_get = Command::new("oc")
        .stdout(Stdio::piped())
        .args(&["get", "imagestream", imagestream])
        .spawn()?;
    Ok(())
}

fn main() {
    let matches = App::new("imagestream-ostree-hash")
        .version("0.1")
        .about("Gather bijective mapping of images and ostree hashes")
        .arg(Arg::with_name("imagestream").required(true))
        .get_matches();

    match run(matches.value_of("imagestream").unwrap()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    }
}
