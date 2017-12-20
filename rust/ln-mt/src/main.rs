use std::path::Path;
use std::{io,process};
extern crate clap;
use clap::{Arg, App};
extern crate openat;
use openat::{SimpleType};
extern crate rayon;
use rayon::prelude::*;

fn linkat_recurse(src_dfd: &openat::Dir,
                  src_name: &Path,
                  dest_dfd: &openat::Dir,
                  dest_name: &Path) -> io::Result<()> {
    let src = src_dfd.sub_dir(src_name)?;
    let src_meta = src.metadata(".")?;

    dest_dfd.create_dir(dest_name, src_meta.stat().st_mode)?;
    let dest = dest_dfd.sub_dir(dest_name)?;
    for entry in src.list_dir(Path::new("."))?.par_iter() {
        let entry = entry?;
        let entrytype: openat::SimpleType =
            match entry.simple_type() {
                Some(t) => t,
                None => {
                    let meta = src.metadata(entry.file_name())?;
                    meta.simple_type()
                }
            };
        let entry_path = Path::new(entry.file_name());
        match entrytype {
            SimpleType::Dir => {
                linkat_recurse(&src, entry_path,
                               &dest, entry_path)?
            },
            _ => {
                openat::hardlink(&src, entry_path,
                                 &dest, entry_path)?
            }
        }
    }
    Ok(())
}

fn run(src_path: &Path, dest_path: &Path) -> io::Result<()> {
    let cwd = openat::Dir::cwd();
    linkat_recurse(&cwd, src_path, &cwd, dest_path)
}

fn main() {
    let matches = App::new("ln-mt")
        .version("0.1")
        .about("Like ln, but multithreaded")
        .arg(Arg::with_name("src").required(true))
        .arg(Arg::with_name("dest").required(true))
        .get_matches();
    let src_path = Path::new(matches.value_of("src").unwrap());
    let dest_path = Path::new(matches.value_of("dest").unwrap());

    ::process::exit(
        match run(src_path, dest_path) {
            Ok(_) => 0,
            Err(e) => { println!("{}", e); 1 }
        })
}
