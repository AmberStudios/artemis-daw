use std::fs;
use std::io;

pub fn read() {
    //here is some nice copypasta straight from the docs

    let filename = "etc/layout.conf";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        //throw error if file not found/not readable
        .expect("[artemis file i/o] Error while reading file");

    println!("With text:\n{}", contents);
}
pub fn remove() -> std::io::Result<()> {
    fs::remove_file("a.txt")?;
    Ok(())
}
pub fn write() -> std::io::Result<()>
{
    let filename = "logs/test.log";

    fs::write(filename, "file write complete")?;
    Ok(())
}
