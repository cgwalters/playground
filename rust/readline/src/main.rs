extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;
use std::ascii::AsciiExt;

fn main() {
    let re = Regex::new(r"[:^alpha:]+").unwrap();
    let mut freqs = HashMap::new();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for word in re.split(&line) {
            *freqs.entry(word.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    let mut freqs: Vec<_> = freqs.iter().collect();
    freqs.sort_by(|&(worda, counta), &(wordb, countb)| {
        (counta, worda).cmp(&(countb, wordb)).reverse()
    });
    
    for &(word, freq) in freqs.iter() {
        println!("{} {}", freq, word);
    }
}
