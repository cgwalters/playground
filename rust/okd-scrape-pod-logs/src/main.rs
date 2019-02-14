use clap::{Arg, App};
use reqwest;
extern crate failure;
use failure::Fallible;
use url::Url;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PR_RE: Regex = Regex::new(r"/build/([^/]+)/(.+)").unwrap();
}

fn gcs_get(client: &Client, bucket: &str, object: &str) -> Fallible<String> {

    reqwest
}

fn main() -> Fallible<()> {
    let matches = App::new("okd-scrape-pod-logs")
        .version("0.1")
        .arg(Arg::with_name("URL").required(true))
        .get_matches();
    let urlstr = matches.value_of("URL").unwrap();
    let url = Url::parse(urlstr)?;
    let path = url.path();
    let caps = PR_RE.captures(path).ok_or_else(|| failure::err_msg("URL does not match OpenShift PR regexp"))?;
    let buf : String = reqwest::get(urlstr)?.text()?;
    println!("{}", buf);
    Ok(())
}
