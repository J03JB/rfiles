use std::io::Result;
use std::process::Command;
// use crate::Result;

pub struct Action {
    pub open_file: fn(&str) -> Result<()>,
    // pub help: Help,
    pub new_folder: Result<()>,
}

pub fn open_file(file_path: &str) -> Result<()> {
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

pub fn new_folder(dir_name: &str) -> Result<()> {
    Command::new("mkdir").arg(dir_name).spawn()?;
    Ok(())
}
