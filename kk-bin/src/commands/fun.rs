use anyhow::bail;

pub fn nop() -> anyhow::Result<()> {
    Ok(())
}

pub fn escape() -> anyhow::Result<()> {
    Ok(())
}
pub fn error() -> anyhow::Result<()> {
    bail!("Just an error  :)")
}
