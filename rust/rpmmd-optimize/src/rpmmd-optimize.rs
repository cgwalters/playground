extern crate clap;
extern crate xml;
#[macro_use]
extern crate failure;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

use failure::Error;
use std::{io, collections::HashMap};
use std::path::Path;

use clap::{App, Arg};
//use xml::writer::EmitterConfig;

mod repomd;
use repomd::*;

#[derive(PartialEq, Eq, Hash)]
pub struct PackageId {
    pub name: String,
    pub epoch: u32,
    pub ver: String,
    pub rel: String,
    pub arch: String,
}

impl PackageId {
    fn new(name: &str, epoch: u32, ver: &str, rel: &str, arch: &str) -> PackageId {
        PackageId { name: name.to_string(),
                    epoch: epoch,
                    ver: ver.to_string(),
                    rel: rel.to_string(),
                    arch: arch.to_string(), }
    }
}

struct PrimaryMd {
    packages: HashMap<PackageId, PackageXml>,
}

impl PrimaryMd {
    fn new() -> Self {
        PrimaryMd { packages: HashMap::new() }
    }
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

fn process_filelists(name: &str, srcp: &Path, r: &RepoDataItem) -> Result<(), Error> {
    let path = repodata_item_path(srcp, &r.location.href)?;
    let inf = std::fs::File::open(*path)?;
    let inf = io::BufReader::new(inf);
    xml_package_stream_map(name, inf, &mut |v| eprintln!("{:?}", v))
}

fn process_primary(name: &str, srcp: &Path, r: &RepoDataItem) -> Result<(), Error> {
    let path = repodata_item_path(srcp, &r.location.href)?;
    let inf = std::fs::File::open(*path)?;
    let inf = io::BufReader::new(inf);
    let mut primarymd = PrimaryMd::new();
    xml_package_stream_map(name, inf, &mut |pkg| {
        for elt in pkg.0 {
            eprintln!("{:?}", elt);
        }
    })
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
            "filelists" => process_filelists("filelists", &srcp, v)?,
            "primary" => process_primary("metadata", &srcp, v)?,
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
