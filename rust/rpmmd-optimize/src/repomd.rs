use failure::Error;
use std;
use xml;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Deserialize)]
pub struct RepoMD {
    pub revision: u64,
    pub data: Vec<RepoDataItem>,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataLocation {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct RepoDataItem {
    #[serde(rename = "type")]
    pub repodatatype: String,
    pub checksum: String,
    #[serde(rename = "open-checksum")]
    pub open_checksum: Option<String>,
    pub location: RepoDataLocation,
    pub timestamp: u64,
    pub size: u64,
    #[serde(rename = "open-size")]
    pub open_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersion {
    pub epoch: u32,
    pub ver: String,
    pub rel: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageTime {
    pub file: u64,
    pub build: u64,
}

#[derive(Debug, Deserialize)]
pub struct PackageSize {
    pub file: u64,
    pub build: u64,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub arch: String,
    pub version: PackageVersion,
    pub summary: String,
    pub description: String,
    pub url: String,
}

// Helper function to iterate over XML stream, discarding events until we find
// our expected element.
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

#[derive(Debug)]
pub struct PackageXml(Vec<XmlEvent>);

// Both primary.xml and filelists.xml have an outer container element, then are
// just an array of <package>.  Invoke a callback for each <package> element, gathering
// a Vec of its elements.
pub fn xml_package_stream_map<R, F>(start_element: &str, input: R, f: &mut F) -> Result<(), Error>
where
    R: std::io::Read,
    F: FnMut(PackageXml) -> (),
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
                f(PackageXml(pkg));
                pkg = Vec::new();
            }
            pkg.push(pkgevent);
        } else {
            break;
        }
    }

    Ok(())
}

