use std::os::unix::fs::MetadataExt;

fn main() -> anyhow::Result<()> {
    let name = std::env::args().nth(1).unwrap();
    let meta = std::fs::metadata(name)?;
    let dev = meta.dev();
    println!("{dev:?}");
    Ok(())
}
