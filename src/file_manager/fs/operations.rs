use anyhow::Result;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::file_manager::tui::Tui;

pub fn open_file<W>(tui: &mut Tui<W>, file_path: &Path) -> Result<()>
where
    W: io::Write,
{
    tui.suspend()?;

    let result = if cfg!(target_os = "macos") {
        Command::new("nvim").arg(file_path).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(file_path).spawn()
        // Command::new("nvim").arg(file_path).spawn()
    } else {
        return Err(anyhow::anyhow!("Unsupported operating system"));
    };

    result?.wait()?;

    tui.resume()?;

    Ok(())
}

pub fn new_folder(dir_name: &Path) -> io::Result<()> {
    Command::new("mkdir").arg("-i").arg(dir_name).spawn()?;
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
