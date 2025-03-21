use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn open_file(file_path: &str) -> io::Result<()> {
    if cfg!(target_os = "macos") {
        Command::new("nvim").arg(file_path).spawn()?;
    } else if cfg!(target_os = "linux") {
        Command::new("$EDITOR").arg(file_path).spawn()?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported operating system",
        ));
    }

    Ok(())
}

pub fn new_folder(dir_name: &str) -> io::Result<()> {
    Command::new("mkdir").arg(dir_name).spawn()?;
    Ok(())
}

pub fn new_file(path: &Path) -> io::Result<()> {
    fs::File::create_new(path)?;
    Ok(())
}

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
