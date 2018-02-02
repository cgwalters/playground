extern crate clap;
extern crate quick_xml;

use std::io::{Result,Error};
use std::{io,thread,fs};
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use quick_xml::reader::Reader;
use quick_xml::events::{Event,BytesStart,BytesEnd};
use quick_xml::writer::Writer;

fn write_package<W : io::Write>(w: &Writer<W>, pkgid: &str, name: &str, arch: &str, epoch: &str,
                    rel: &str, files: &Vec<String>, dirs: &Vec<String>) -> io::Result<()> {
    w.write_event(Event::Start(BytesStart::owned(b"package".into(), 7)))?;
    w.write_event(Event::End(BytesEnd::owned(b"package")))?;
    Ok(())
}

fn run(in_path: &str, out_path: &str) -> io::Result<()> {
    let inf_raw = std::fs::File::open(in_path)?;
    let inf = io::BufReader::new(inf_raw);
    let outf_raw = std::fs::File::create(out_path)?;
    let outf = io::BufWriter::new(outf_raw);
    let mut reader = Reader::from_reader(inf);
    let mut buf = Vec::new();
    let mut writer = Writer::new(outf);
    loop {
        let mut pkgid = "";
        let mut name = "";
        let mut arch = "";
        let mut epoch = "";
        let mut ver = "";
        let mut rel = "";
        let mut files : Vec<String> = vec![];
        let mut dirs : Vec<String> = vec![];
        // For <file>
        let mut in_file = false;
        let mut is_dir = false;
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"filelists" => writer.write_event(Event::Start(e)),
                    b"file" => {
                        is_dir = e.attributes().any(|a| a.key == b"dir");
                        in_file = true;
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"filelists" => writer.write_event(Event::End(e)),
                    b"file" => { in_file = false },
                    b"package" => {
                        write_package(pkgid, name, arch, epoch, ver, rel, files, dirs);
                        pkgid = "";
                        name = "";
                        arch = "";
                        ver = "";
                        rel = "";
                        files = vec![];
                        dirs = vec![];
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let decoded = e.unescape_and_decode(&reader).map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
                if in_file {
                    if is_dir {
                        dirs.push(decoded);
                    } else {
                        files.push(decoded);
                    }
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }

    Ok(())
}

fn main() {
    let matches = App::new("rpmmd-optimize")
        .version("0.1")
        .about("Apply optimization passes to rpm-md")
        .arg(Arg::with_name("filelists-in").required(true))
        .arg(Arg::with_name("filelists-out").required(true))
        .get_matches();

    match run(matches.value_of("filelists-in").unwrap(),
              matches.value_of("filelists-out").unwrap()) {
        Ok(_) => { },
        Err(e) => { eprintln!("{:?}", e);
                    std::process::exit(1) }
    }
}
