// Copyright 2015 Colin Walters <walters@verbum.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(libc)]

use std::env;
use std::thread;
use std::ffi::CString;
extern crate libc;
use libc::setenv;

fn main() {
   for i in 0..4 {
       thread::spawn(move || {
           loop {
               let v = env::var("FOO");
           }
       });
   }
   for i in 0..100000000 {
     let k = CString::new(format!("FOO{}", i)).unwrap();
     let v = CString::new(format!("{}", i)).unwrap();
     unsafe {
       setenv(k.as_ptr(), v.as_ptr(), 1);
     }
   }
}
