extern crate clap;
extern crate xml;

use std::io::{Read,Write,Result,Error};
use std::{io,thread,fs};
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, EmitterConfig};

//fn write_package<W: Write>(w: &mut EventWriter<W>, pkgid: &str, name: &str, arch: &str, epoch: &str,
//                    rel: &str, files: &Vec<String>, dirs: &Vec<String>) -> io::Result<()> {
//    w.write(XmlEvent::start_element("package"));
//    w.write(XmlEvent::end_element("package"));
//    Ok(())
//}

fn write_event<W: Write>(writer: &mut EventWriter<W>, event: xml::writer::events::XmlEvent) -> io::Result<()> {
    writer.write(event).map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
}

fn run(in_path: &str, out_path: &str) -> io::Result<()> {
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
            _ => (), // There are several other `Event`s we do not consider here
        }
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
