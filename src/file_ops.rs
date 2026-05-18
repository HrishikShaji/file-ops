use std::fs;
use std::io;

pub fn create_file(name: &str) -> io::Result<()> {
    fs::write(name, "")?;
    Ok(())
}

pub fn write_file(name: &str, content: &str) -> io::Result<()> {
    fs::write(name, content)?;
    Ok(())
}

pub fn read_file(name: &str) -> io::Result<String> {
    fs::read_to_string(name)
}

pub fn delete_file(name: &str) -> io::Result<()> {
    fs::remove_file(name)?;
    Ok(())
}
