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

fn await_start_element<I>(start_element: &str, events: &mut I) -> Result<Option<XmlEvent>, Error>
where
    I: Iterator<Item = Result<XmlEvent, xml::reader::Error>>,
{
    for event in events {
        let event = event?;
        let matches = match &event {
            XmlEvent::StartElement { name, .. } => &name.local_name[..] == start_element,
            _ => false,
        };
        if matches {
            return Ok(Some(event));
        }
    }
    Ok(None)
}

fn xml_package_stream_map<R, F>(start_element: &str, input: R, f: &mut F) -> Result<(), Error>
where
    R: std::io::Read,
    F: FnMut(&[XmlEvent]) -> (),
{
    let parser = EventReader::new(input);
    let mut events = parser.into_iter();
    match await_start_element(start_element, &mut events)? {
        None => bail!(r#"End of stream, expected "{}""#, start_element),
        _ => {}
    };
    let mut pkg: Vec<XmlEvent> = Vec::new();
    loop {
        if let Some(pkgevent) = await_start_element("package", &mut events)? {
            if pkg.len() > 0 {
                f(&pkg[..]);
                pkg.clear();
            }
            pkg.push(pkgevent);
        } else {
            break;
        }
    }

    Ok(())
}

fn repodata_item_path<P: AsRef<Path>>(
    srcp: P,
    href: &str,
) -> Result<Box<std::path::PathBuf>, Error> {
    let srcp = srcp.as_ref();
    if let Some(path) = href.rsplit('/').next() {
        Ok(Box::new(srcp.clone().join(path)))
    } else {
        bail!("Invalid href: \"{}\"", href)
    }
}

fn process(name: &str, srcp: &Path, r: &RepoDataItem) -> Result<(), Error> {
    let path = repodata_item_path(srcp, &r.location.href)?;
    let inf = std::fs::File::open(*path)?;
    let inf = io::BufReader::new(inf);
    xml_package_stream_map(name, inf, &mut |v| eprintln!("{:?}", v))
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
            "filelists" => process("filelists", &srcp, v)?,
            "primary" => process("metadata", &srcp, v)?,
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
