use std::fs;
use std::path::{Path, PathBuf};
use std::io;

pub fn copy_file(source: &Path, destination: &Path) -> io::Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}

pub fn move_file(source: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(source, destination)?;
    Ok(())
}

pub fn delete_file(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}
