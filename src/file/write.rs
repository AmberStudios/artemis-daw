pub fn write() -> std::io::Result<()>
{
    let filename = "logs/test.log";

    fs::write(filename, "file write complete")?;
    Ok(())
}
