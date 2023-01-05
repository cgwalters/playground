fn main() -> anyhow::Result<()> {
    fail::fail_point!("main", true, |msg| {
        let msg = msg.as_deref().unwrap_or("synthetic error");
        Err(anyhow::anyhow!("{msg}"))
    });
    Ok(())
}
