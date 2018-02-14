extern crate clap;
extern crate xml;
extern crate serde_xml_rs;
#[macro_use] extern crate serde_derive;

use std::io::{Read,Write,Result,Error};
use std::{io,thread,fs};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, EmitterConfig};
use serde_xml_rs::deserialize;

#[derive(Debug, Deserialize)]
struct RepoMD {
    revision : u64,
    data: Vec<RepoDataItem>,
}

#[derive(Debug, Deserialize)]
struct RepoDataLocation {
    href: String,
}

#[derive(Debug, Deserialize)]
struct RepoDataItem {
    checksum: String,
    #[serde(rename = "open-checksum")]
    open_checksum: Option<String>,
    location: RepoDataLocation,
    timestamp: u64,
    size: u64,
    #[serde(rename = "open-size")]
    open_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct PackageVersion {
    epoch: u32,
    ver: String,
    rel: String
}

struct PackageId {
    name: String,
    epoch: u32,
    ver: String,
    rel: String,
    arch: String,
}

fn write_event<W: Write>(writer: &mut EventWriter<W>, event: xml::writer::events::XmlEvent) -> io::Result<()> {
    writer.write(event).map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
}

fn filter_filelists(in_path: &str, out_path: &str) -> io::Result<()> {
    let inf = std::fs::File::open(in_path)?;
    let inf = io::BufReader::new(inf);
    let outf = std::fs::File::create(out_path)?;
    let parser = EventReader::new(inf);
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&outf);

    // For <file>
    let mut in_file = false;
    let mut is_dir = false;
    // Loop
    for event in parser {
        let event = event.map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        match &event {
            &XmlEvent::StartElement { name : ref eltname, ref attributes, .. } => {
                match &eltname.local_name[..] {
                    "file" => {
                        is_dir = attributes.iter().any(|a| a.name.local_name == "dir");
                        in_file = true;
                    }
                    _ => {
                        if let Some(we) = event.as_writer_event() {
                            write_event(&mut writer, we);
                        }
                    }
                }
            },
            &XmlEvent::EndElement { name : ref eltname } => {
                match &eltname.local_name[..] {
                    "file" => { in_file = false },
                    _ => {
                        if let Some(we) = event.as_writer_event() {
                            write_event(&mut writer, we);
                        }
                    }
                }
            },
            &XmlEvent::Characters(ref txt) => {
                if in_file {
                    if is_dir {
                        println!("dir={}", txt);
                    } else {
                        println!("file={}", txt);
                    }
                }
            },
            _ => ()
        }
    }

    Ok(())
}

fn run(srcdir: &str, destdir: &str) -> io::Result<()> {
    let srcp = Path::new(srcdir);
    let destp = Path::new(destdir);

    let repodata_p = srcp.join("repomd.xml");
    let repodata_in = std::fs::File::open(repodata_p)?;
    let repodata_in = io::BufReader::new(repodata_in);
    let repomd : RepoMD = serde_xml_rs::deserialize(repodata_in).
        map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
    Ok(())
}

fn main() {
    let matches = App::new("rpmmd-optimize")
        .version("0.1")
        .about("Apply optimization passes to rpm-md")
        .arg(Arg::with_name("srcdir").required(true))
        .arg(Arg::with_name("destdir").required(true))
        .get_matches();

    match run(matches.value_of("srcdir").unwrap(),
              matches.value_of("destdir").unwrap()) {
        Ok(_) => { },
        Err(e) => { eprintln!("{:?}", e);
                    std::process::exit(1) }
    }
}
