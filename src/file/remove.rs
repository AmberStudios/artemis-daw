pub fn remove() -> std::io::Result<()> {
    fs::remove_file("a.txt")?;
    Ok(())
}
