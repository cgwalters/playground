extern crate clap;
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use failure::Error;
use clap::{App, Arg};

#[derive(Deserialize, Debug)]
struct ImageStreamTagItem {
    created: String,
    #[serde(rename = "dockerImageReference")]
    docker_image_reference: String,
    image: String,
}

#[derive(Deserialize, Debug)]
struct ImageStreamTag {
    tag: String,
    items: Vec<ImageStreamTagItem>
}

#[derive(Deserialize, Debug)]
struct ImageStreamStatus {
    #[serde(rename = "dockerImageRepository")]
    docker_image_repository: String,
    #[serde(rename = "publicDockerImageRepository")]
    public_docker_image_repository: String,
    tags: Vec<ImageStreamTag>,
}

#[derive(Deserialize, Debug)]
struct ImageStream {
    status: ImageStreamStatus,
}

#[derive(Deserialize, Debug)]
struct SkopeoInspect {
    #[serde(rename = "Labels")]
    labels: BTreeMap<String, String>,
}

#[derive(Serialize, Debug)]
struct ContainerToOstree(BTreeMap<String, Option<String>>);

fn run(imagestream: &str) -> Result<(), Error> {
    let oc_get = Command::new("oc")
        .stdout(Stdio::piped())
        .args(&["get", "-o", "json", "imagestream", imagestream])
        .spawn()?;
    let json_in = oc_get.stdout.unwrap();
    let is : ImageStream = serde_json::from_reader(json_in)?;

    let mut oscontainer_to_ostree  = ContainerToOstree(BTreeMap::new());

    let private_base = &is.status.docker_image_repository;
    let public_base = &is.status.public_docker_image_repository;
    for tag in &is.status.tags {
        for item in &tag.items {
            let mut public_ref : String = item.docker_image_reference.trim_left_matches(private_base).into();
            public_ref.insert_str(0, public_base);
            public_ref.insert_str(0, "docker://");
            let skopeo_proc = Command::new("skopeo")
                .stdout(Stdio::piped())
                .args(&["inspect", public_ref.as_str()])
                .spawn()?;
            let skopeo_in = skopeo_proc.stdout.unwrap();
            let inspect : SkopeoInspect = serde_json::from_reader(skopeo_in)?;
            let ostree_hash = inspect.labels.get("io.openshift.os-commit");
            oscontainer_to_ostree.0.insert(public_ref.clone(), ostree_hash.map(Clone::clone));
        }
    }
    { let stdout = std::io::stdout();
      let mut handle = stdout.lock();
      serde_json::to_writer_pretty(handle, &oscontainer_to_ostree);
    }
    Ok(())
}

fn main() {
    let matches = App::new("imagestream-ostree-hash")
        .version("0.1")
        .about("Gather bijective mapping of images and ostree hashes")
        .arg(Arg::with_name("imagestream").required(true))
        .get_matches();

    match run(matches.value_of("imagestream").unwrap()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    }
}
