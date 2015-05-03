// Copyright 2015 Colin Walters <walters@verbum.org>
//
// Compare two passwd/group files and report on differences.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_serialize;
extern crate docopt;

use std::collections::HashMap;
use std::io::BufRead;
use std::fs::File;

use docopt::Docopt;

static USAGE: &'static str = "
Usage: passwd-diff <passwd> <group>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_passwd: String,
    arg_group: String
}

trait PwGrEntry {
    fn name(&self) -> &str;
}

#[derive(Debug)]
struct PwEntry {
    name: String,
    uid: u32,
    gid: u32
}

impl PwGrEntry for PwEntry {
    fn name(&self) -> &str { return &self.name }
}

#[derive(Debug)]
struct GrEntry {
    name: String,
    gid: u32
}

impl PwGrEntry for GrEntry {
    fn name(&self) -> &str { return &self.name }
}

fn parse_pw_entry(string: &str) -> Result<PwEntry, String> {
   let name: &str;
   let uid: u32;
   let gid: u32;
   let mut parts = string.splitn(5, ":");
   match parts.next() {
       Some(v) => name = v,
       None => return Err("Invalid entry".to_string())
   }
   match parts.next() {
       Some(_) => (),
       None => return Err("Invalid entry".to_string())
   }
   match parts.next() {
       Some(v) => uid =
           match v.parse::<u32>() {
               Ok(n) => n,
               Err(e) => return Err(format!("Failed to parse uid '{}': {}", v, e))
           },
       None => return Err("Invalid entry".to_string())
   }
   match parts.next() {
       Some(v) => gid =
           match v.parse::<u32>() {
               Ok(n) => n,
               Err(e) => return Err(format!("Failed to parse gid '{}': {}", v, e))
           },
       None => return Err("Invalid entry".to_string())
   }
   return Ok(PwEntry { name: name.to_string(), uid: uid, gid: gid });
}

fn parse_gr_entry(string: &str) -> Result<GrEntry, String> {
   let name: &str;
   let gid: u32;
   let mut parts = string.splitn(4, ":");
   match parts.next() {
       Some(v) => name = v,
       None => return Err("Invalid entry".to_string())
   }
   match parts.next() {
       Some(_) => (),
       None => return Err("Invalid entry".to_string())
   }
   match parts.next() {
       Some(v) => gid =
           match v.parse::<u32>() {
               Ok(n) => n,
               Err(e) => return Err(format!("Failed to parse gid '{}': {}", v, e))
           },
       None => return Err("Invalid entry".to_string())
   }
   return Ok(GrEntry { name: name.to_string(), gid: gid });
}

fn parse_pw_or_gr_file<T: PwGrEntry>(input: File, f: Box<Fn(&str) -> Result<T, String>>) -> HashMap<String, T> {
    let reader = std::io::BufReader::new(input);
    let mut r: HashMap<String, T> = HashMap::new();
    for lv in reader.lines() {
        let line = lv.unwrap();
        let p : T = f(&line).unwrap();
        r.insert(p.name().to_string(), p);
    }
    return r;
}

fn parse_passwd_path(path: &str) -> HashMap<String, PwEntry> {
    let s = File::open(path).unwrap();
    return parse_pw_or_gr_file::<PwEntry>(s, Box::new(parse_pw_entry));
}

fn parse_group_path(path: &str) -> HashMap<String, GrEntry> {
    let s = File::open(path).unwrap();
    return parse_pw_or_gr_file::<GrEntry>(s, Box::new(parse_gr_entry));
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let pw = parse_passwd_path(&args.arg_passwd);
    let gr = parse_group_path(&args.arg_group);
    for (key, val) in pw.iter() {
        match gr.get(key) {
            Some(v) => {
                if v.gid != val.gid {
                    println!("mismatch on group id for '{}': {} != {}", key, v.gid, val.gid);
                }
            },
            None => { println!("group file missing {}", key) }
        }
    }
}
