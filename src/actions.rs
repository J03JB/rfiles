use std::process::Command;
use std::io::Result;
// use crate::Result;

pub struct Action {
    pub open_file: fn(&str) -> Result<()>,
}

pub fn open_file(file_path: &str) -> Result<()> {
    if cfg!(target_os = "macos") {
        Command::new("nvim")
            .arg(file_path)
            .spawn()?;
    } else if cfg!(target_os = "linux") {
        Command::new("$EDITOR")
            .arg(file_path)
            .spawn()?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported operating system",
        ));
    }

    Ok(())
}

