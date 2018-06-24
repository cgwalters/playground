extern crate clap;
extern crate xml;
#[macro_use]
extern crate failure;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

use failure::{err_msg, Error};
use std::io;
use std::path::Path;

use clap::{App, Arg};
use xml::reader::{EventReader, XmlEvent};
use xml::writer::EmitterConfig;

mod repomd;
use repomd::*;

fn filter_filelists<S: AsRef<Path>, D: AsRef<Path>>(in_path: S, out_path: D) -> Result<(), Error> {
    let in_path = in_path.as_ref();
    let out_path = out_path.as_ref();
    eprintln!("src: {:?} dest: {:?}", in_path, out_path);
    let inf = std::fs::File::open(in_path)?;
    let inf = io::BufReader::new(inf);
    let out_filelists = out_path.clone().join("filelists.xml");
    let outf = std::fs::File::create(out_filelists)?;
    let parser = EventReader::new(inf);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&outf);

    // For <file>
    let mut in_file = false;
    let mut is_dir = false;
    // Loop
    for event in parser {
        let event = event?;
        match &event {
            &XmlEvent::StartElement {
                name: ref eltname,
                ref attributes,
                ..
            } => match &eltname.local_name[..] {
                "file" => {
                    is_dir = attributes.iter().any(|a| a.name.local_name == "dir");
                    in_file = true;
                }
                _ => {
                    if let Some(we) = event.as_writer_event() {
                        writer.write(we).map_err(err_msg)?
                    }
                }
            },
            &XmlEvent::EndElement { name: ref eltname } => match &eltname.local_name[..] {
                "file" => in_file = false,
                _ => {
                    if let Some(we) = event.as_writer_event() {
                        writer.write(we).map_err(err_msg)?
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
            }
            _ => (),
        }
    }

    Ok(())
}

fn run(srcdir: &str, destdir: &str) -> Result<(), Error> {
    let srcp = Path::new(srcdir);
    let destp = Path::new(destdir);
    let dest_repodatap = destp.clone().join("repodata");
    std::fs::create_dir(&dest_repodatap)?;

    let src_repomd_p = srcp.join("repomd.xml");
    let repodata_in = std::fs::File::open(src_repomd_p)?;
    let repodata_in = io::BufReader::new(repodata_in);
    let repomd: RepoMD = serde_xml_rs::deserialize(repodata_in)?;
    eprintln!("{:?}", repomd);
    for v in &repomd.data {
        match v.repodatatype.as_str() {
            "filelists" => {
                if let Some(filelist_path) = v.location.href.rsplit('/').next() {
                    let filelist_path = srcp.clone().join(filelist_path);
                    filter_filelists(&filelist_path, &dest_repodatap)?
                } else {
                    bail!("Invalid filelists href \"{}\"", v.location.href);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("rpmmd-optimize")
        .version("0.1")
        .about("Apply optimization passes to rpm-md")
        .arg(Arg::with_name("srcdir").required(true))
        .arg(Arg::with_name("destdir").required(true))
        .get_matches();

    match run(
        matches.value_of("srcdir").unwrap(),
        matches.value_of("destdir").unwrap(),
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    }
}
