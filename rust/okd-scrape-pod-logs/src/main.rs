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

fn gcs_get(client: &reqwest::Client, bucket: &str, object: &str) -> Fallible<String> {
    let baseurl = Url::parse("https://www.googleapis.com/").unwrap();
    let path = format!("storage/v1/b/{}/{}", bucket, object);
    let url = baseurl.join(&path)?;
    (|| {
        client.get(url.as_str()).send()?.text()
    })().map_err(|e|failure::err_msg(e.to_string()))
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
    let client = reqwest::Client::new();
    let buf = gcs_get(&client, caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str())?;
    println!("{}", buf);
    Ok(())
}
