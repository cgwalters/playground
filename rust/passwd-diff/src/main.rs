// Copyright 2015 Colin Walters <walters@verbum.org>
//
// Sanity check two passwd/group files, or
// compare two passwd/group files and report on differences.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_serialize;
extern crate docopt;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::BufRead;
use std::fs::File;

use docopt::Docopt;

static USAGE: &'static str = "
Usage:
  passwd-diff --pw-gr-check <passwd> <group>
  passwd-diff --pw-gr-diff <passwd> <group> <new-passwd> <new-group>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_pw_gr_check: bool,
    flag_pw_gr_diff: bool,
    arg_passwd: String,
    arg_group: String,
    arg_new_passwd: String,
    arg_new_group: String
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

fn pw_gr_check(passwd: &str, group: &str) {
    let pw = parse_passwd_path(passwd);
    let gr = parse_group_path(group);
    let mut pw_reverse : HashMap<u32, &str> = HashMap::new();
    let mut gr_reverse : HashMap<u32, &str> = HashMap::new();
    for (key, val) in pw.iter() {
        match gr.get(key) {
            Some(v) => {
                if v.gid != val.gid {
                    println!("mismatch on group id for '{}': {} != {}", key, v.gid, val.gid);
                }
            },
            None => { println!("group file missing {}", key) }
        }
        match pw_reverse.entry(val.uid) {
            Occupied(entry) => { println!("uid duplicated between {} and {}", entry.get(), key) },
	    Vacant(entry) => { entry.insert(&key); () }
        }
    }
    for (key, val) in gr.iter() {
        match pw.get(key) {
            None => println!("passwd file missing {}", key),
            _ => ()
        }
        match gr_reverse.entry(val.gid) {
            Occupied(entry) => { println!("gid duplicated between {} and {}", entry.get(), key) },
	    Vacant(entry) => { entry.insert(&key); () }
        }
    }
}

fn pw_diff(passwd: &str, new_passwd: &str) {
    let old_pw = parse_passwd_path(passwd);
    let new_pw = parse_passwd_path(new_passwd);

    for (key, oval) in old_pw.iter() {
        match new_pw.get(key) {
            Some(nval) => {
                if oval.uid != nval.uid {
                    println!("error: mismatch on passwd uid for '{}': {} != {}", key, oval.uid, nval.uid);
                }
                if oval.gid != nval.gid {
                    println!("error: mismatch on passwd gid for '{}': {} != {}", key, oval.gid, nval.gid);
                }
            },
            None => { println!("error: user removed: {}", key) }
        }
    } 
    for (key, _) in new_pw.iter() {
        match old_pw.get(key) {
            Some(_) => (),
            None => { println!("user added: {}", key) }
        }
    } 
}

fn gr_diff(group: &str, new_group: &str) {
    let old_gr = parse_group_path(group);
    let new_gr = parse_group_path(new_group);

    for (key, oval) in old_gr.iter() {
        match new_gr.get(key) {
            Some(nval) => {
                if oval.gid != nval.gid {
                    println!("error: mismatch on group gid for '{}': {} != {}", key, oval.gid, nval.gid);
                }
                if oval.gid != nval.gid {
                    println!("error: mismatch on group gid for '{}': {} != {}", key, oval.gid, nval.gid);
                }
            },
            None => { println!("error: group removed: {}", key) }
        }
    } 
    for (key, _) in new_gr.iter() {
        match old_gr.get(key) {
            Some(_) => (),
            None => { println!("group added: {}", key) }
        }
    } 
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_pw_gr_check {
        pw_gr_check(&args.arg_passwd, &args.arg_group);
    } else if args.flag_pw_gr_diff {
        pw_diff(&args.arg_passwd, &args.arg_new_passwd);
 	gr_diff(&args.arg_group, &args.arg_new_group);
    }
}
