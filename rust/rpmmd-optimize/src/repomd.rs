use failure::{Error, err_msg};
use std::io::{Read,Write};
use std::{io,thread,fs};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App};
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EventWriter, EmitterConfig};
use serde_xml_rs::deserialize;

#[derive(Debug, Deserialize)]
pub struct RepoMD {
    revision : u64,
    data: Vec<RepoDataItem>,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataLocation {
    href: String,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataItem {
    #[serde(rename = "type")]
    repodatatype: String,
    checksum: String,
    #[serde(rename = "open-checksum")]
    open_checksum: Option<String>,
    location: RepoDataLocation,
    timestamp: u64,
    size: u64,
    #[serde(rename = "open-size")]
    open_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    epoch: u32,
    ver: String,
    rel: String
}

pub struct PackageId {
    name: String,
    epoch: u32,
    ver: String,
    rel: String,
    arch: String,
}
